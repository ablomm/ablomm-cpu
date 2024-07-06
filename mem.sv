module mem #(
    parameter integer WORD_SIZE = 32,
	parameter integer ADDR_WIDTH = 16,
    parameter integer DEPTH = 2 ** ADDR_WIDTH
) (
	input clk,
    input [WORD_SIZE-1:0] data,
    input [ADDR_WIDTH-1:0] addr,
    output tri [WORD_SIZE-1:0] out,
    input rd,
    input wr
);

  logic [WORD_SIZE-1:0] mem[DEPTH-1];

  assign out = rd ? mem[addr] : 'hz;

  always_ff @(posedge clk) begin
    if (wr) mem[addr] <= data;
  end
endmodule
