module offset_filter #(
    parameter integer WIDTH = 32,
    parameter integer OFFSET_WIDTH = 32
) (
    output tri [WIDTH-1:0] out,
    input [WIDTH-1:0] in,
    input signed [OFFSET_WIDTH-1:0] offset
);
  // for whatever reason, an explicit wire is required
  // (it wont sign extend if you just put in offset directly)
  wire [WIDTH-1:0] extended_offset = offset;
  assign out = in + extended_offset;
endmodule
