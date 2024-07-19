module register_file #(
    parameter integer WORD_SIZE = 32,
    parameter integer SEL_WIDTH = 8,
    parameter integer COUNT_WIDTH = 8,
    parameter integer DEPTH = 2 ** SEL_WIDTH

) (
    input clk,
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

  logic [WORD_SIZE-1:0] registers[DEPTH];

  initial begin
    int i;
    for (i = 0; i < DEPTH; i = i + 1) begin
      registers[i] = 0;
    end
  end

  assign a = oe_a ? registers[sel_a] : 'hz;
  assign b = oe_b ? registers[sel_b] : 'hz;

  always @(posedge clk) begin
    if (ld) registers[sel_in] = in;
    if (post_count_a) registers[sel_a] += WORD_SIZE'(signed'(count_a));
    if (post_count_b) registers[sel_b] += WORD_SIZE'(signed'(count_b));
  end

  always @(posedge pre_count_a) registers[sel_a] += WORD_SIZE'(signed'(count_a));
  always @(posedge pre_count_b) registers[sel_b] += WORD_SIZE'(signed'(count_b));

endmodule
