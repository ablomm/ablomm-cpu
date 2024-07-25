module cu_tb;
  import cu_pkg::*;
  import reg_pkg::*;

  logic clk;
  logic start;
  logic hwint;
  logic rst = 0;
  ir_t ir;
  status_t status;

  // control signals
  logic mem_rd;
  logic mem_wr;

  logic oe_alu;
  logic [3:0] alu_op;

  logic [31:0] a_reg_mask;
  logic [31:0] b_reg_mask;

  reg_e sel_a_reg;
  reg_e sel_b_reg;
  reg_e sel_in_reg;

  logic oe_a_reg_file;
  logic oe_b_reg_file;
  logic ld_reg_file;
  logic post_inc_sp;
  logic pre_dec_sp;
  logic post_inc_pc;

  logic oe_a_consts;
  logic oe_b_consts;

  logic oe_a_ir;
  logic oe_b_ir;
  logic ld_ir;

  logic ld_status;
  logic oe_a_status;
  logic oe_b_status;

  logic ld_alu_status;
  logic imask_in;
  logic ld_imask;
  cpu_mode_e mode_in;
  logic ld_mode;

  cu cu0 (
      .*,
      .clk(~clk)
  );


  initial begin
    #200;
    $display("\ntesting cu");
    test_start;
    test_alu(alu_pkg::AND);
    test_fetch;
    test_alu(alu_pkg::SHL);
    test_fetch;
  end

  task static test_start;
    begin
      // STOP state
      clk = 0;
      #1;

      start = 1;
      clk   = 1;
      #1;

      test_fetch;
      start = 0;
    end
  endtask

  task static test_fetch;
    begin
      // FETCH state
      clk = 0;
      #1;

      $display("sel_b_reg: %d, oe_b_reg_file: %d, mem_rd: %d, ld_ir: %d, post_inc_pc: %d",
               sel_b_reg, oe_b_reg_file, mem_rd, ld_ir, post_inc_pc);

      assert (sel_b_reg === reg_pkg::PC && oe_b_reg_file === 1 && mem_rd === 1 && post_inc_pc === 1);
    end
  endtask

  task static test_alu(input alu_op_e op_in, input logic reverse_in = 0, input logic load_in = 1,
                       input logic set_status_in = 0, input reg_a_in = reg_pkg::R0,
                       input reg_b_in = reg_pkg::R1, input reg_c_in = reg_pkg::R2);
    begin
      ir.condition = NONE;
      ir.instruction = op_in;
      ir.params.alu_op.flags.immediate = 0;
      ir.params.alu_op.flags.reverse = reverse_in;
      ir.params.alu_op.flags.load = load_in;
      ir.params.alu_op.flags.set_status = set_status_in;
      ir.params.alu_op.reg_a = reg_a_in;
      ir.params.alu_op.reg_b = reg_b_in;
      ir.params.alu_op.reg_c = reg_c_in;

      clk = 1;
      #1;

      // AND state
      clk = 0;
      #1;

      $display(
          "sel_a_reg: %d, oe_a_reg_file: %d, sel_b_reg: %d, oe_b_reg_file: %d, alu_op: %d, sel_in_reg: %d, ld_reg_file: %d",
          sel_a_reg, oe_a_reg_file, sel_b_reg, oe_b_reg_file, alu_op, sel_in_reg, ld_reg_file);

      assert (sel_a_reg === reg_b_in && oe_a_reg_file === 1 && sel_b_reg === reg_c_in && oe_b_reg_file === 1 && alu_op === op_in && sel_in_reg === reg_a_in && ld_reg_file === 1);

      clk = 1;
      #1;
    end
  endtask
endmodule
