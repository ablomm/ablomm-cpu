module mem #(
    parameter integer WORD_SIZE = 32,
    parameter integer DEPTH = 2 ** 16
) (
	input clk,
    input rd,
    input wr,
    input [15:0] addr,
    input [31:0] data,
    output tri [31:0] out
);

  logic [WORD_SIZE-1:0] mem[DEPTH-1];

  assign out = rd ? mem[addr] : 'hz;

  always_ff @(posedge clk) begin
    if (wr) mem[addr] <= data;
  end
endmodule
