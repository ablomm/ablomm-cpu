// program counter
module pc_reg #(
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
    input post_inc,
    output logic [SIZE-1:0] value = INITIAL_VAL // only if you need to directly access (not on the data/addr bus)
);
  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always @(posedge clk or posedge rst)
    if (rst) value <= INITIAL_VAL;
    else value <= (ld ? in : value) + SIZE'(post_inc);
endmodule
