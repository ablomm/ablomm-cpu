import reg_pkg::*;

module mask_filter #(
    parameter integer WIDTH = 32
) (
    output tri [WIDTH-1:0] out,
    input [WIDTH-1:0] in,
    input reg_mask_e mask
);
  logic [31:0] mask_32;
  assign mask_32 = reg_pkg::mask_32(mask);
  assign out = in & mask_32;
endmodule
