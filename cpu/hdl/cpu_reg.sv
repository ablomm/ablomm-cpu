module cpu_reg #(
    parameter integer SIZE = 32,
    parameter logic [SIZE-1:0] INITIAL_VAL = 0
) (
    input clk,
    input rst,
    output tri [SIZE-1:0] a,
    output tri [SIZE-1:0] b,
    input [SIZE-1:0] in,
    input oe_a,
    input oe_b,
    input ld,
    output logic [SIZE-1:0] value = INITIAL_VAL // only if you need to directly access (not on the data/addr bus)
);

  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always_ff @(posedge clk or posedge rst)
    if (rst) value <= INITIAL_VAL;
    else if (ld) value <= in;
endmodule
