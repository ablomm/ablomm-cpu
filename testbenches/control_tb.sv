module control_tb;
  logic clk;
  logic [31:0] ir;

  // control signals
  wire pc_rst;
  wire inc_pc;
  wire oe_mdr;
  wire oe_data_reg_file;
  wire [7:0] sel_data_reg_file;
  wire oe_mar;
  wire oe_addr_reg_file;
  wire [7:0] sel_addr_reg_file;
  wire oe_pc;
  wire oe_alu;
  wire [3:0] alu_op;
  wire ld_reg_file;
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
      .oe_data_reg_file(oe_data_reg_file),
      .sel_data_reg_file(sel_data_reg_file),
      .oe_mar(oe_mar),
      .oe_addr_reg_file(oe_addr_reg_file),
      .sel_addr_reg_file(sel_addr_reg_file),
      .oe_pc(oe_pc),
      .oe_alu(oe_alu),
      .alu_op(alu_op),
      .mem_rd(mem_rd),
      .mem_wr(mem_wr),
      .ld_reg_file(ld_reg_file),
      .ld_ir(ld_ir),
      .ld_pc(ld_pc),
      .ld_a(ld_a),
      .ld_b(ld_b),
      .ld_status(ld_status),
      .ld_mdr(ld_mdr),
      .ld_mar(ld_mar)
  );


endmodule
