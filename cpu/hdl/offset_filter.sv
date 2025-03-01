module offset_filter #(
    parameter integer WIDTH = 32,
    parameter integer OFFSET_WIDTH = 32
) (
    output tri [WIDTH-1:0] out,
    input [WIDTH-1:0] in,
    input signed [OFFSET_WIDTH-1:0] offset
);
  assign out = in + WIDTH'(offset);
endmodule
