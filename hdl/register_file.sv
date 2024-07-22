module register_file #(
    parameter integer WORD_SIZE = 32,
    parameter integer SEL_WIDTH = 4,
    parameter integer DEPTH = 2 ** SEL_WIDTH

) (
    input clk,
    input rst,
    output tri [WORD_SIZE-1:0] a,
    output tri [WORD_SIZE-1:0] b,
    input [WORD_SIZE-1:0] in,
    input oe_a,
    input oe_b,
    input ld,
    input [SEL_WIDTH-1:0] sel_a,
    input [SEL_WIDTH-1:0] sel_b,
    input [SEL_WIDTH-1:0] sel_in,
    input post_inc_sp,
    input pre_dec_sp,
    input post_inc_pc
);
  genvar i;
  generate
    for (i = 0; i < DEPTH - 2; i++) begin : g_registers
      cpu_reg #(
          .SIZE(WORD_SIZE)
      ) register (
          .*,
          .oe_a(sel_a === i && oe_a),
          .oe_b(sel_b === i && oe_b),
          .ld(sel_in === i && ld),
          .value()
      );
    end
  endgenerate

  sp_reg #(
      .SIZE(WORD_SIZE)
  ) sp (
      .*,
      .oe_a(sel_a === DEPTH - 2 && oe_a),
      .oe_b(sel_b === DEPTH - 2 && oe_b),
      .ld(sel_in === DEPTH - 2 && ld),
      .post_inc(post_inc_sp),
      .pre_dec(pre_dec_sp),
      .value()
  );

  pc_reg #(
      .SIZE(WORD_SIZE)
  ) pc (
      .*,
      .oe_a(sel_a === DEPTH - 1 && oe_a),
      .oe_b(sel_b === DEPTH - 1 && oe_b),
      .ld(sel_in === DEPTH - 1 && ld),
      .post_inc(post_inc_pc),
      .value()
  );

endmodule
