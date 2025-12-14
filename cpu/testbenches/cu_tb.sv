module cu_tb;
  import cu_pkg::*;
  import reg_pkg::*;

  logic clk = 0;
  logic en = 0;
  logic irq = 0;
  logic rst = 0;
  ir_t ir;
  status_t status;

  // control signals
  logic rd;
  logic wr;

  logic oe_alu;
  logic [3:0] alu_op;

  reg_mask_e a_reg_mask;
  reg_mask_e b_reg_mask;
  logic en_offset;

  reg_e sel_a_reg;
  reg_e sel_b_reg;
  reg_e sel_in_reg;

  logic oe_a_reg;
  logic oe_b_reg;
  logic ld_reg;

  logic post_inc_sp;
  logic pre_dec_sp;
  logic post_inc_pc;

  logic oe_a_consts;
  logic oe_b_consts;

  logic oe_a_ir;
  logic oe_b_ir;
  logic ld_ir;

  logic ld_alu_status;
  logic imask_in;
  logic ld_imask;
  cpu_mode_e mode_in;
  logic ld_mode;

  cu cu0 (.*);


  initial begin
    #300;
    $display("\ntesting cu");

    en = 1;
    #1;

    test_alu(alu_pkg::AND);
    test_alu(alu_pkg::SHL);
  end

  task static next_state;
    begin
      clk = 0;
      #1;
      clk = 1;
      #1;
    end
  endtask

  task static test_fetch;
    begin
      $display("sel_b_reg: %d, oe_b_reg_file: %d, rd: %d, ld_ir: %d, post_inc_pc: %d", sel_b_reg,
               oe_b_reg, rd, ld_ir, post_inc_pc);

      assert (sel_b_reg === reg_pkg::PC && oe_b_reg === 1 && rd === 1 && post_inc_pc === 1)
      else $fatal;
    end
  endtask

  task static test_alu(input alu_op_e op_in, input logic reverse_in = 0, input logic loadn_in = 0,
                       input logic set_status_in = 0, input reg_e reg_a_in = reg_pkg::R0,
                       input reg_e reg_b_in = reg_pkg::R1, input reg_e reg_c_in = reg_pkg::R2);
    begin
      // FETCH
      test_fetch;
      ir.condition = NONE;
      ir.instruction = instruction_e'(8'(op_in) | 8'hf0);  // alu ops start with 0xf*
      ir.operands.alu_op.flags.immediate = 0;
      ir.operands.alu_op.flags.reverse = reverse_in;
      ir.operands.alu_op.flags.loadn = loadn_in;
      ir.operands.alu_op.flags.set_status = set_status_in;
      ir.operands.alu_op.reg_a = reg_a_in;
      ir.operands.alu_op.reg_b = reg_b_in;
      ir.operands.alu_op.reg_c = reg_c_in;

      next_state;
      // DECODE

      next_state;
      // ALU

      $display(
          "sel_a_reg: %d, oe_a_reg_file: %d, sel_b_reg: %d, oe_b_reg_file: %d, alu_op: %d, sel_in_reg: %d, ld_reg_file: %d",
          sel_a_reg, oe_a_reg, sel_b_reg, oe_b_reg, alu_op, sel_in_reg, ld_reg);

      assert (sel_a_reg === reg_b_in && oe_a_reg === 1 && sel_b_reg === reg_c_in && oe_b_reg === 1 && alu_op === op_in && sel_in_reg === reg_a_in && ld_reg === 1)
      else $fatal;

      next_state;
      // FETCH
    end
  endtask
endmodule
