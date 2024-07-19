module tty #(
    parameter integer WORD_SIZE = 32,
    parameter integer ADDR_WIDTH = 16,
    parameter integer DEPTH = 2 ** ADDR_WIDTH
) (
    input clk,
    input [WORD_SIZE-1:0] data,
    input wr,
    input en
);

  always @(posedge clk) begin
    if (en && wr) $write("%s", data[7:0]);
  end
endmodule
