module cpu_reg #(
    parameter integer SIZE = 32,
    parameter integer COUNT_WIDTH = 8,
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
    input logic [COUNT_WIDTH-1:0] count,
    input pre_count,
    input post_count,
    output logic [SIZE-1:0] value = INITIAL_VAL  // only if you need to direclty access (not on the data/addr bus)
);

  always @(posedge rst) value <= INITIAL_VAL;

  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always @(posedge clk) begin
    if (ld) value = in;
    if (post_count) value += SIZE'(signed'(count));
  end

  always @(posedge pre_count) value += SIZE'(signed'(count));

endmodule
