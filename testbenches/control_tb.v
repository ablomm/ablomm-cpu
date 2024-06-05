module control_tb;
  reg clk = 0;
  reg [31:0] ir;

  // control signals
  wire pc_rst;
  wire inc_pc;
  wire oe_mdr;
  wire oe_a_data;
  wire oe_b_data;
  wire oe_mar;
  wire oe_pc;
  wire oe_a_addr;
  wire oe_b_addr;
  wire oe_alu;
  wire [3:0] alu_op;
  wire ld_ir;
  wire ld_pc;
  wire ld_a;
  wire ld_b;
  wire ld_status;
  wire ld_mdr;
  wire ld_mar;

  control control0 (
      .clk(clk),
      .ir(ir),
      .pc_rst(pc_rst),
      .inc_pc(inc_pc),
      .oe_mdr(oe_mdr),
      .oe_a_data(oe_a_data),
      .oe_b_data(oe_b_data),
      .oe_mar(oe_mar),
      .oe_pc(oe_pc),
      .oe_a_addr(oe_a_addr),
      .oe_b_addr(oe_b_addr),
      .oe_alu(oe_alu),
      .alu_op(alu_op),
      .mem_rd(mem_rd),
      .mem_wr(mem_wr),
      .ld_ir(ld_ir),
      .ld_pc(ld_pc),
      .ld_a(ld_a),
      .ld_b(ld_b),
      .ld_status(ld_status),
      .ld_mdr(ld_mdr),
      .ld_mar(ld_mar)
  );
  always #5 clk = ~clk;

  always begin
	#4;
	#10;
  end

endmodule
