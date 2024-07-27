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

  wire oe_a_reg;
  wire oe_b_reg;
  wire ld_reg;

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
      .clk(~clk) // negative clk so that control signals are created before loads (fixes race conditions)
  );

  wire alu_status_t alu_status;
  alu alu0 (
      .oe(oe_alu),
      .operation(alu_op),
      .carry_in(1'(status.alu_status.carry)),
      .a(a_bus),
      .b(b_bus),
      .out(result_bus),
      .status(alu_status)
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
  // 0-12 => general registers (including fp)
  register_file #(
      .DEPTH(13)
  ) reg_file (
      .clk(clk),
      .rst(rst),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .in(result_bus),
      .oe_a(oe_a_reg),
      .oe_b(oe_b_reg),
      .ld(ld_reg),
      .sel_a(sel_a_reg),
      .sel_b(sel_b_reg),
      .sel_in(sel_in_reg)
  );

  wire status_t status;
  status_reg status_reg (
      .clk(clk),
      .rst(rst),
      .a(a_reg_bus[5:0]),
      .b(b_reg_bus[5:0]),
      .in(result_bus[5:0]),
      .oe_a(sel_a_reg === reg_pkg::STATUS && oe_a_reg),
      .oe_b(sel_b_reg === reg_pkg::STATUS && oe_b_reg),
      .ld(sel_in_reg === reg_pkg::STATUS && ld_reg),
      .alu_status_in(alu_status),
      .ld_alu_status(ld_alu_status),
      .value(status)
  );

  sp_reg sp (
      .clk(clk),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .in(result_bus),
      .oe_a(sel_a_reg === reg_pkg::SP && oe_a_reg),
      .oe_b(sel_b_reg === reg_pkg::SP && oe_b_reg),
      .ld(sel_in_reg === reg_pkg::SP && ld_reg),
      .post_inc(post_inc_sp),
      .pre_dec(pre_dec_sp),
      .value()
  );

  pc_reg pc (
      .clk(clk),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .in(result_bus),
      .oe_a(sel_a_reg === reg_pkg::PC && oe_a_reg),
      .oe_b(sel_b_reg === reg_pkg::PC && oe_b_reg),
      .ld(sel_in_reg === reg_pkg::PC && ld_reg),
      .post_inc(post_inc_pc),
      .value()
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
  wire ir_t ir;
  cpu_reg ir_reg (
      .clk(clk),
      .rst(rst),
      .a(a_reg_bus),
      .b(b_reg_bus),
      .in(result_bus),
      .oe_a(oe_a_ir),
      .oe_b(oe_b_ir),
      .ld(ld_ir),
      .value(ir)
  );
endmodule
