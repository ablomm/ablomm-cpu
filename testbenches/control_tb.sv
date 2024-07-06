module control_tb;
  logic clk;
  logic [31:0] ir_value;
  logic [3:0] status_value;

  logic mem_rd;
  logic mem_wr;

  // control signals
  logic oe_a_reg_file;
  logic oe_b_reg_file;
  logic ld_reg_file;
  logic [3:0] sel_a_reg_file;
  logic [3:0] sel_b_reg_file;
  logic [7:0] count_a_reg_file;
  logic [7:0] count_b_reg_file;

  logic ld_ir;

  logic ld_status;

  logic oe_mdr;
  logic ld_mdr;

  logic oe_mar;
  logic ld_mar;

  logic oe_alu;
  logic [3:0] alu_op;

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


endmodule
