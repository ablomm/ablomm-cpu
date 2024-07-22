import cu_pkg::*;
import alu_pkg::*;
import reg_pkg::*;

module cpu (
    input clk,
    input start,
    input rst,
    input hwint,
    output tri [31:0] a_bus,
    output tri [31:0] b_bus,
    output tri [31:0] result_bus,
    output mem_rd,
    output mem_wr
);

  // control signals
  wire oe_alu;
  wire alu_op_e alu_op;

  wire [31:0] a_reg_mask;
  wire [31:0] b_reg_mask;

  wire reg_e sel_a_reg;
  wire reg_e sel_b_reg;
  wire reg_e sel_in_reg;

  wire oe_a_reg_file;
  wire oe_b_reg_file;
  wire ld_reg_file;
  wire post_inc_sp;
  wire pre_dec_sp;
  wire post_inc_pc;

  wire oe_a_consts;
  wire oe_b_consts;

  wire oe_a_ir;
  wire oe_b_ir;
  wire ld_ir;

  wire ld_alu_status;
  wire imask_in;
  wire ld_imask;
  wire cpu_mode_e mode_in;
  wire ld_mode;

  cu cu0 (
      .*,
      .clk(~clk), // negative clk so that control signals are created before loads (fixes race conditions)
      .ir(ir_value),
      .status(status_value)
  );

  wire alu_status_t alu_status_in;
  alu alu0 (
      .oe(oe_alu),
      .operation(alu_op),
      .carry_in(1'(status_value.alu_status.carry)),
      .a(a_bus),
      .b(b_bus),
      .out(result_bus),
      .status(alu_status_in)
  );

  tri [31:0] a_reg_bus;
  filter filter_a (
      .out (a_bus),
      .in  (a_reg_bus),
      .mask(a_reg_mask)
  );

  tri [31:0] b_reg_bus;
  filter filter_b (
      .out (b_bus),
      .in  (b_reg_bus),
      .mask(b_reg_mask)
  );

  // public registers
  // 0-12 => general registers
  // 13 => fp
  // 14 => sp
  // 15 => pc
  register_file reg_file (
      .clk(clk),
      .rst(rst),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .in(result_bus),
      .oe_a(oe_a_reg_file),
      .oe_b(oe_b_reg_file),
      .ld(ld_reg_file),
      .sel_a(sel_a_reg),
      .sel_b(sel_b_reg),
      .sel_in(sel_in_reg),
      .post_inc_sp(post_inc_sp),
      .pre_dec_sp(pre_dec_sp),
      .post_inc_pc(post_inc_pc)
  );

  reg_constants reg_consts (
      .clk(clk),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .oe_a(oe_a_consts),
      .oe_b(oe_b_consts),
      .sel_a(sel_a_reg),
      .sel_b(sel_b_reg)
  );

  // internal private registers
  wire ir_t ir_value;
  cpu_reg ir (
      .clk(clk),
      .rst(rst),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .in(result_bus),
      .oe_a(oe_a_ir),
      .oe_b(oe_b_ir),
      .ld(ld_ir),
      .value(ir_value)
  );

  wire status_t status_value;
  status_reg status (
      .*,
      .value(status_value)
  );
endmodule
