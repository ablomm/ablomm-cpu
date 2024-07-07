module ir #(
    parameter integer SIZE = 32
) (
    input clk,
    output tri [SIZE-1:0] a,
    output tri [SIZE-1:0] b,
    input [SIZE-1:0] in,
    input oe_a_8,
    input oe_a_16,
    input oe_b_8,
    input oe_b_16,
    input ld,
    output logic [SIZE-1:0] value
);

  cpu_reg #(
      .SIZE(SIZE)
  ) reg0 (
      .clk(clk),
      .value(value),
      .in(in),
      .ld(ld)
  );

  assign a   = oe_a_8 ? value & 8'hff : 'hz;
  assign a   = oe_a_16 ? value & 16'hffff : 'hz;
  assign b   = oe_b_8 ? value & 8'hff : 'hz;
  assign b   = oe_b_16 ? value & 16'hffff : 'hz;
endmodule
