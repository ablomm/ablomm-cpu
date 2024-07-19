module cu_tb;
  import cpu_pkg::*;
  import reg_pkg::*;

  logic clk;
  logic start;
  ir_t ir_value;
  status_t status_value;

  logic mem_rd;
  logic mem_wr;

  logic [31:0] a_reg_mask;
  logic [31:0] b_reg_mask;

  // control signals
  logic oe_a_reg_file;
  logic oe_b_reg_file;
  logic ld_reg_file;
  reg_e sel_a_reg_file;
  reg_e sel_b_reg_file;
  reg_e sel_in_reg_file;
  logic [7:0] count_a_reg_file;
  logic [7:0] count_b_reg_file;
  logic pre_count_a_reg_file;
  logic pre_count_b_reg_file;
  logic post_count_a_reg_file;
  logic post_count_b_reg_file;

  logic oe_a_ir;
  logic oe_b_ir;
  logic ld_ir;

  logic ld_status;

  logic oe_mdr;
  logic ld_mdr;

  logic oe_mar;
  logic ld_mar;

  logic oe_alu;
  logic [3:0] alu_op;

  cu cu0 (
      .*,
      .ir(ir_value),
      .status(status_value)
  );


  initial begin
    // STOP state
    clk = 0;
    #1;

    start = 1;
    clk   = 1;
    #1;

    // FETCH state
    clk = 0;
    #1;

    clk   = 1;
    start = 0;
    $display(
        "sel_b_reg_file: %d, oe_b_reg_file: %d, mem_rd: %d, ld_ir: %d, count_b_reg_file: %d, post_count_b_reg_file",
        sel_b_reg_file, oe_b_reg_file, mem_rd, ld_ir, count_b_reg_file, post_count_b_reg_file);

    if (sel_b_reg_file !== reg_pkg::PC || oe_b_reg_file !== 1 || mem_rd !== 1 || count_b_reg_file !== 1 || post_count_b_reg_file !== 1)
      $finish(1);
    ir_value.condition = NONE;
    ir_value.instruction = cpu_pkg::AND;
    ir_value.params.and_params.reg_a = reg_pkg::R0;
    ir_value.params.and_params.reg_b = reg_pkg::R1;
    ir_value.params.and_params.reg_c = reg_pkg::R2;
    #1;

    // AND state
    clk = 0;
    #1;
    clk = 1;
    $display(
        "sel_a_reg_file: %d, oe_a_reg_file: %d, sel_b_reg_file: %d, oe_b_reg_file: %d, alu_op: %d, sel_ing_reg_file: %d, ld_reg_file: %d",
        sel_a_reg_file, oe_a_reg_file, sel_b_reg_file, oe_b_reg_file, alu_op, sel_in_reg_file,
        ld_reg_file);

    if (sel_a_reg_file !== reg_pkg::R1 || oe_a_reg_file !== 1 || sel_b_reg_file !== reg_pkg::R2 || oe_b_reg_file !== 1 || alu_op !== alu_pkg::AND || sel_in_reg_file !== reg_pkg::R0 || ld_reg_file !== 1)
      $finish(1);

  end
endmodule
