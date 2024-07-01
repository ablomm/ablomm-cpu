module cpu (
    input clk,
    output tri [31:0] data_bus,
    output tri [31:0] addr_bus,
    output tri [31:0] result_bus,
    output mem_rd,
    output mem_wr
);

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
      .ir(ir_value),
      .status(status_value),
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

  register_file reg_file (
      .clk(clk),
      .oe_a(oe_data_reg_file),
      .oe_b(oe_addr_reg_file),
      .ld(ld_reg_file),
      .sel_a(sel_data_reg_file),
      .sel_b(sel_addr_reg_file),
      .input_bus(result_bus),
      .a_bus(data_bus),
      .b_bus(addr_bus)
  );

  wire [31:0] ir_value;
  cpu_reg ir (
      .clk(clk),
      .result_bus(result_bus),
      .ld(ld_ir),
      .value(ir_value)
  );

  wire [2:0] status_value;
  cpu_reg #(
      .SIZE(3)
  ) status (
      .clk(clk),
      .result_bus(alu_status_out),
      .ld(ld_status),
      .value(status_value)
  );

  cpu_reg mdr (
      .clk(clk),
      .data_bus(data_bus),
      .result_bus(result_bus),
      .oe_data(oe_mdr),
      .ld(ld_mdr)
  );

  cpu_reg mar (
      .clk(clk),
      .addr_bus(addr_bus),
      .result_bus(result_bus),
      .oe_addr(oe_mar),
      .ld(ld_mar)
  );

  counter pc (
      .clk(clk),
      .rst(pc_rst),
      .cnt(inc_pc),
      .ld (ld_pc),
      .oe (oe_pc),
      .in (result_bus),
      .out(addr_bus)
  );

  wire [2:0] alu_status_out;
  alu alu0 (
      .oe(oe_alu),
      .operation(alu_op),
      .carry_in(status_value[0]),
      .a(data_bus),
      .b(addr_bus),
      .out(result_bus),
      .status(alu_status_out)
  );

endmodule
