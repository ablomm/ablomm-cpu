module filter #(
    parameter integer SIZE = 32
) (
    output tri [SIZE-1:0] a_out,
    output tri [SIZE-1:0] b_out,
    input [SIZE-1:0] a_in,
    input [SIZE-1:0] b_in,
    input [SIZE-1:0] a_mask,
    input [SIZE-1:0] b_mask
);

  assign a_out = a_in & a_mask;
  assign b_out = b_in & b_mask;
endmodule
