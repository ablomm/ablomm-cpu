module cpu_reg #(
    parameter integer SIZE = 32
) (
    input clk,
    output tri [SIZE-1:0] a,
    output tri [SIZE-1:0] b,
    input [SIZE-1:0] in,
    input oe_a,
    input oe_b,
    input ld,
    output logic [SIZE-1:0] value  // only if you need to direclty access (not on the data/addr bus)
);

  assign out   = value;
  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always_ff @(posedge clk) begin
    if (ld) value <= in;
  end

endmodule
