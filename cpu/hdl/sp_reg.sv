// stack pointer
module sp_reg #(
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
    input pre_dec,
    output [SIZE-1:0] value  // only if you need to directly access (not on the data/addr bus)
);
  logic [SIZE-1:0] value_reg = INITIAL_VAL;

  // I have to do this instead of just having an `always @(pre_dec) value_reg -= 1;
  // because apparently it's not good to have both blocking and nonblocking assignments
  assign value = value_reg - SIZE'(pre_dec);
  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always_ff @(posedge clk or posedge rst)
    if (rst) value_reg <= INITIAL_VAL;
    else value_reg <= (ld ? in : value_reg) + SIZE'(post_inc) - SIZE'(pre_dec);
endmodule
