module cpu (
    input clk,
    output tri [31:0] a_bus,
    output tri [31:0] b_bus,
    output tri [31:0] result_bus,
    output mem_rd,
    output mem_wr
);

  // control signals
  wire oe_a_reg_file;
  wire oe_b_reg_file;
  wire ld_a_reg_file;
  wire ld_b_reg_file;
  wire [3:0] sel_a_reg_file;
  wire [3:0] sel_b_reg_file;
  wire [7:0] count_a_reg_file;
  wire [7:0] count_b_reg_file;

  wire ld_ir;

  wire ld_status;

  wire oe_mdr;
  wire ld_mdr;

  wire oe_mar;
  wire ld_mar;

  wire oe_alu;
  wire [3:0] alu_op;

  control control0 (
      .clk(clk),
      .ir(ir_value),
      .status(status_value),

      .mem_rd(mem_rd),
      .mem_wr(mem_wr),

      .oe_a_reg_file(oe_a_reg_file),
      .oe_b_reg_file(oe_b_reg_file),
      .ld_reg_file(ld_reg_file),
      .sel_a_reg_file(sel_a_reg_file),
      .sel_b_reg_file(sel_b_reg_file),
	  .count_a_reg_file(count_a_reg_file),
	  .count_b_reg_file(count_b_reg_file),

      .ld_ir(ld_ir),

      .ld_status(ld_status),

      .oe_mdr(oe_mdr),
      .ld_mdr(ld_mdr),

      .oe_mar(oe_mar),
      .ld_mar(ld_mar),

      .oe_alu(oe_alu),
      .alu_op(alu_op)
  );

  wire [3:0] alu_status_out;
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
      .a(a_bus),
      .b(b_bus),
      .in(result_bus),
      .oe_a(oe_a_reg_file),
      .oe_b(oe_b_reg_file),
      .ld_a(ld_a_reg_file),
      .ld_b(ld_b_reg_file),
      .sel_a(sel_a_reg_file),
      .sel_b(sel_b_reg_file),
      .count_a(count_a_reg_file),
      .count_b(count_b_reg_file)
  );

  // internal private registers
  wire [31:0] ir_value;
  cpu_reg ir (
      .clk(clk),
      .in(result_bus),
      .ld(ld_ir),
      .value(ir_value)
  );

  wire [3:0] status_value;
  cpu_reg #(
      .SIZE(4)
  ) status (
      .clk(clk),
      .in(alu_status_out),
      .ld(ld_status),
      .value(status_value)
  );

  cpu_reg mdr (
      .clk(clk),
      .a(a_bus),
      .in(result_bus),
      .oe_a(oe_mdr),
      .ld(ld_mdr)
  );

  cpu_reg mar (
      .clk(clk),
      .b(b_bus),
      .in(result_bus),
      .oe_b(oe_mar),
      .ld(ld_mar)
  );

endmodule
