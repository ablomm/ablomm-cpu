module register_file #(
    parameter integer WORD_SIZE = 32,
    parameter integer SEL_WIDTH = 8,
    parameter integer COUNT_WIDTH = 8,
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
    input [COUNT_WIDTH-1:0] count_a,
    input [COUNT_WIDTH-1:0] count_b,
    input pre_count_a,
    input pre_count_b,
    input post_count_a,
    input post_count_b
);

  genvar i;
  generate
    for (i = 0; i < DEPTH; i++) begin : g_registers
      cpu_reg #(
          .SIZE(WORD_SIZE),
          .COUNT_WIDTH(COUNT_WIDTH)
      ) register (
          .*,
          .oe_a(sel_a === i && oe_a),
          .oe_b(sel_b === i && oe_b),
          .ld(sel_in === i && ld),
          .count(sel_a === i ? count_a : count_b),
          .pre_count((sel_a === i && pre_count_a) || (sel_b === i && pre_count_b)),
          .post_count((sel_a === i && post_count_a) || (sel_b === i && post_count_b)),
          .value()
      );
    end
  endgenerate
endmodule
