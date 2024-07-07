module filter #(
    parameter integer SIZE = 32
) (
    output tri [SIZE-1:0] out,
    input [SIZE-1:0] in,
    input [SIZE-1:0] mask
);

  assign out = in & mask;
endmodule
