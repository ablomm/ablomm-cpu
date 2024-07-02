module counter #(
    parameter integer SIZE = 32
) (
    input clk,
    input rst,
    input cnt,
    input ld,
    input oe,
    input [SIZE-1:0] in,
    output tri [SIZE-1:0] out
);

  reg [SIZE-1:0] value;

  assign out = oe ? value : 'hz;

  always_ff @(posedge clk) begin
    if (cnt) value <= value + 1;
    if (ld) value <= in;
  end

  always @(posedge rst) value <= 0;

endmodule
