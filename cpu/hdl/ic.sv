// super basic interupt conroller; basically just an AND gate
module ic #(
    parameter integer IRQ_LENGTH = 16
) (
    input clk,
    input [IRQ_LENGTH-1:0] irq_in,
    output tri [IRQ_LENGTH-1:0] out,
    input rd,
    output irq_out
);

  // for some reason verilator complains when it's 'hz
  assign out = rd ? irq_in : {IRQ_LENGTH{1'hz}};
  assign irq_out = irq_in !== 'b0;
endmodule
