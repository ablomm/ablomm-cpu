// super basic interupt conroller; basically just an AND gate
module ic #(
    parameter integer WORD_SIZE  = 32,
    parameter integer IRQ_LENGTH = 16
) (
    input clk,
    input [IRQ_LENGTH-1:0] irq_in,
    output tri [WORD_SIZE-1:0] out,
    input rd,
    output irq_out
);
  assign out = rd ? irq_in : 'hz;
  assign irq_out = irq_in !== 'b0;
endmodule
