module reg_constants #(
    parameter integer WORD_SIZE = 32,
    parameter integer SEL_WIDTH = 4,
    parameter integer DEPTH = 2 ** SEL_WIDTH

) (
    input clk,
    output tri [WORD_SIZE-1:0] a,
    output tri [WORD_SIZE-1:0] b,
    input oe_a,
    input oe_b,
    input [SEL_WIDTH-1:0] sel_a,
    input [SEL_WIDTH-1:0] sel_b
);

  wire [WORD_SIZE-1:0] constants[DEPTH];
  assign constants[0] = 0;
  assign constants[1] = 1;  // HWINT vector
  assign constants[2] = 2;  // SWINT vector
  assign constants[3] = 3;	// EXCEPT vector

  assign a = oe_a ? constants[sel_a] : 'hz;
  assign b = oe_b ? constants[sel_b] : 'hz;

endmodule
