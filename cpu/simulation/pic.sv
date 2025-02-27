module pic #(
    parameter integer WORD_SIZE  = 32,
    parameter integer IRQ_LENGTH = 16
) (
    input clk,
    input [IRQ_LENGTH-1:0] irq,
    output tri [WORD_SIZE-1:0] out,
    input rd,
    output intr
);
  assign out  = rd ? irq : 'hz;
  assign intr = irq !== 'b0;
endmodule
