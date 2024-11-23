module mask_filter #(
    parameter integer WIDTH = 32
) (
    output tri [WIDTH-1:0] out,
    input [WIDTH-1:0] in,
    input [WIDTH-1:0] mask
);

  assign out = in & mask;
endmodule
