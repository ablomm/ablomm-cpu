import cpu_pkg::*;
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
  tri [31:0] a_reg_bus;
  tri [31:0] b_reg_bus;

  // control signals
  wire oe_alu;
  wire alu_op_e alu_op;

  wire [31:0] a_reg_mask;
  wire [31:0] b_reg_mask;

  wire oe_a_reg_file;
  wire oe_b_reg_file;
  wire ld_reg_file;
  wire reg_e sel_a_reg;
  wire reg_e sel_b_reg;
  wire reg_e sel_in_reg;
  wire [7:0] count_a_reg;
  wire [7:0] count_b_reg;
  wire pre_count_a_reg_file;
  wire pre_count_b_reg_file;
  wire post_count_a_reg_file;
  wire post_count_b_reg_file;

  wire oe_a_consts;
  wire oe_b_consts;

  wire oe_a_ir;
  wire oe_b_ir;
  wire ld_ir;

  wire ld_status;

  wire cpu_mode_e mode_in;
  wire ld_mode;

  wire int_mask_in;
  wire ld_int_mask;

  cu cu0 (
      .*,
      .clk(~clk), // negative clk so that control signals are created before loads (fixes race conditions)
      .ir(ir_value),
      .status(status_value),
      .mode(mode_value),
      .int_mask(int_mask_value)
  );

  wire status_t alu_status_out;
  alu alu0 (
      .oe(oe_alu),
      .operation(alu_op),
      .carry_in(status_value[0]),
      .a(a_bus),
      .b(b_bus),
      .out(result_bus),
      .status(alu_status_out)
  );

  filter filter_a (
      .out (a_bus),
      .in  (a_reg_bus),
      .mask(a_reg_mask)
  );

  filter filter_b (
      .out (b_bus),
      .in  (b_reg_bus),
      .mask(b_reg_mask)
  );

  // public registers
  // 0-12 => general registers
  // 13 => pc
  // 14 => sp
  // 15 => fp
  register_file #(
      .SEL_WIDTH(4)
  ) reg_file (
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
      .count_a(count_a_reg),
      .count_b(count_b_reg),
      .pre_count_a(pre_count_a_reg_file),
      .pre_count_b(pre_count_b_reg_file),
      .post_count_a(post_count_a_reg_file),
      .post_count_b(post_count_b_reg_file)
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
  cpu_reg #(
      .SIZE(4)
  ) status (
      .clk(clk),
      .rst(rst),
      .in(alu_status_out),
      .ld(ld_status),
      .value(status_value)
  );

  wire mode_value;
  cpu_reg #(
      .SIZE(1)
  ) mode (
      .clk(clk),
      .rst(rst),
      .in(mode_in),
      .ld(ld_mode),
      .value(mode_value)
  );

  wire int_mask_value;
  cpu_reg #(
      .SIZE(1)
  ) int_mask (
      .clk(clk),
      .rst(rst),
      .in(int_mask_in),
      .ld(ld_int_mask),
      .value(int_mask_value)
  );
endmodule
