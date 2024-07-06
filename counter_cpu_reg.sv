module counter_cpu_reg #(
    parameter integer SIZE  = 32,
    parameter integer COUNT = 1
) (
    input clk,
    output tri [SIZE-1:0] a,
    output tri [SIZE-1:0] b,
    input [SIZE-1:0] in,
    input oe_a,
    input oe_b,
    input ld,
    output logic [SIZE-1:0] value,
    input rst,
    input cnt
);

  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always_ff @(posedge clk) begin
    if (cnt) value <= value + COUNT;
    if (ld) value <= in;
  end

  always @(posedge rst) value <= 0;

endmodule
