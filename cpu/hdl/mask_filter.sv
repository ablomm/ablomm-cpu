module mask_filter
  import reg_pkg::*;
#(
    parameter integer WIDTH = 32
) (
    output tri [WIDTH-1:0] out,
    input [WIDTH-1:0] in,
    input reg_mask_e mask
);
  wire [31:0] mask_32 = reg_pkg::get_mask_32(mask);
  assign out = in & mask_32;
endmodule
