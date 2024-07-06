module register_file #(
    parameter integer WORD_SIZE = 32,
    parameter integer SEL_WIDTH  = 8,
	parameter integer COUNT_WIDTH = 8,
    parameter integer DEPTH  = 2 ** SEL_WIDTH

) (
    input clk,
    output tri [WORD_SIZE-1:0] a,
    output tri [WORD_SIZE-1:0] b,
    input [WORD_SIZE-1:0] in,
    input oe_a,
    input oe_b,
    input ld_a,
    input ld_b,
    input [SEL_WIDTH-1:0] sel_a,
    input [SEL_WIDTH-1:0] sel_b,
	input [COUNT_WIDTH-1:0] count_a,
	input [COUNT_WIDTH-1:0] count_b
);

  logic [WORD_SIZE-1:0] registers[DEPTH-1];

  assign a = oe_a ? registers[sel_a] : 'hz;
  assign b = oe_b ? registers[sel_b] : 'hz;

  always @(posedge clk) begin
    if (ld_a) registers[sel_a] = in;
    if (ld_b) registers[sel_b] = in;

	registers[sel_a] = registers[sel_a] + WORD_SIZE'(signed'(count_a));
	registers[sel_b] = registers[sel_b] + WORD_SIZE'(signed'(count_b));
  end

endmodule
