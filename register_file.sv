module register_file #(
    parameter integer WORD_SIZE = 32,
    parameter integer SEL_WIDTH  = 8,
    parameter integer DEPTH  = 2 ** SEL_WIDTH

) (
    input clk,
    output tri [WORD_SIZE-1:0] a,
    output tri [WORD_SIZE-1:0] b,
    input [WORD_SIZE-1:0] in,
    input oe_a,
    input oe_b,
    input ld,
    input [SEL_WIDTH-1:0] sel_a,
    input [SEL_WIDTH-1:0] sel_b
);

  logic [WORD_SIZE-1:0] registers[DEPTH-1];

  assign a = oe_a ? registers[sel_a] : 'hz;
  assign b = oe_b ? registers[sel_b] : 'hz;

  always_ff @(posedge clk) begin
    if (ld) registers[sel_a] <= in;  // ignore sel_b
  end

endmodule
