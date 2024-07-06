module counter #(
    parameter integer SIZE = 32,
	parameter integer COUNT = 1
) (
    input clk,
    input [SIZE-1:0] in,
    output tri [SIZE-1:0] out,
    input oe,
    input ld,
    input rst,
    input cnt
);

  logic [SIZE-1:0] value;

  assign out = oe ? value : 'hz;

  always_ff @(posedge clk) begin
    if (cnt) value <= value + COUNT;
    if (ld) value <= in;
  end

  always @(posedge rst) value <= 0;

endmodule
