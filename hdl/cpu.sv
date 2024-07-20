import cpu_pkg::*;
import alu_pkg::*;
import reg_pkg::*;

module cpu (
    input clk,
    input start,
	input rst,
    output tri [31:0] a_bus,
    output tri [31:0] b_bus,
    output tri [31:0] result_bus,
    output mem_rd,
    output mem_wr
);
  tri [31:0] a_reg_bus;
  tri [31:0] b_reg_bus;

  // control signals
  wire [31:0] a_reg_mask;
  wire [31:0] b_reg_mask;

  wire oe_a_reg_file;
  wire oe_b_reg_file;
  wire ld_reg_file;
  wire reg_e sel_a_reg_file;
  wire reg_e sel_b_reg_file;
  wire reg_e sel_in_reg_file;
  wire [7:0] count_a_reg_file;
  wire [7:0] count_b_reg_file;
  wire pre_count_a_reg_file;
  wire pre_count_b_reg_file;
  wire post_count_a_reg_file;
  wire post_count_b_reg_file;

  wire oe_a_ir;
  wire oe_b_ir;
  wire ld_ir;

  wire ld_status;

  wire oe_mdr;
  wire ld_mdr;

  wire oe_mar;
  wire ld_mar;

  wire oe_alu;
  wire alu_op_e alu_op;

  cu cu0 (
      .*,
      .clk(~clk), // negative clk so that control signals are created before loads (fixes race conditions)
      .ir(ir_value),
      .status(status_value)
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
      .sel_a(sel_a_reg_file),
      .sel_b(sel_b_reg_file),
      .sel_in(sel_in_reg_file),
      .count_a(count_a_reg_file),
      .count_b(count_b_reg_file),
      .pre_count_a(pre_count_a_reg_file),
      .pre_count_b(pre_count_b_reg_file),
      .post_count_a(post_count_a_reg_file),
      .post_count_b(post_count_b_reg_file)
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
endmodule
