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
  wire ld_reg_file;
  wire [3:0] sel_a_reg_file;
  wire [3:0] sel_b_reg_file;

  wire ld_ir;

  wire ld_status;

  wire oe_mdr;
  wire ld_mdr;

  wire oe_mar;
  wire ld_mar;

  wire oe_a_pc;
  wire oe_b_pc;
  wire ld_pc;
  wire rst_pc;
  wire inc_pc;

  wire oe_a_sp;
  wire oe_b_sp;
  wire ld_sp;
  wire rst_sp;
  wire dec_sp;

  wire oe_a_fp;
  wire oe_b_fp;
  wire ld_fp;

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

      .ld_ir(ld_ir),

      .ld_status(ld_status),

      .oe_mdr(oe_mdr),
      .ld_mdr(ld_mdr),

      .oe_mar(oe_mar),
      .ld_mar(ld_mar),

      .oe_a_pc(oe_a_pc),
      .oe_b_pc(oe_b_pc),
      .ld_pc  (ld_pc),
      .rst_pc (rst_pc),
      .inc_pc (inc_pc),

      .oe_a_sp(oe_a_sp),
      .oe_b_sp(oe_b_sp),
      .ld_sp  (ld_sp),
      .rst_sp (rst_sp),
      .dec_sp (dec_sp),

      .oe_a_fp(oe_a_fp),
      .oe_b_fp(oe_b_fp),
      .ld_fp  (ld_fp),

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

  // general registers
  register_file #(
      .SEL_WIDTH(4)
  ) reg_file (
      .clk(clk),
      .a(a_bus),
      .b(b_bus),
      .in(result_bus),
      .oe_a(oe_a_reg_file),
      .oe_b(oe_b_reg_file),
      .ld(ld_reg_file),
      .sel_a(sel_a_reg_file),
      .sel_b(sel_b_reg_file)
  );

  // special purpose registers
  counter_cpu_reg pc (
      .clk(clk),
      .in(result_bus),
      .a(a_bus),
      .b(b_bus),
      .oe_a(oe_a_pc),
      .oe_b(oe_b_pc),
      .ld(ld_pc),
      .rst(rst_pc),
      .cnt(inc_pc)
  );

  counter_cpu_reg #(
      .COUNT(-1)
  ) sp (
      .clk(clk),
      .a(a_bus),
      .b(b_bus),
      .in(result_bus),
      .oe_a(oe_a_sp),
      .oe_b(oe_b_sp),
      .ld(ld_sp),
      .rst(rst_sp),
      .cnt(dec_sp)
  );

  cpu_reg fp (
      .clk(clk),
      .a(a_bus),
      .b(b_bus),
      .in(result_bus),
      .oe_a(oe_a_fp),
      .oe_b(oe_b_fp),
      .ld(ld_fp)
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
