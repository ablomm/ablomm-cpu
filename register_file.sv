module register_file #(
    parameter integer WORLD_SIZE = 32,
    parameter integer SEL_WIDTH  = 8
) (
    input clk,
    input oe_a,
    input oe_b,
    input ld,
    input [SEL_WIDTH-1:0] sel_a,
    input [SEL_WIDTH-1:0] sel_b,
    input [WORLD_SIZE-1:0] input_bus,
    output tri [WORLD_SIZE-1:0] a_bus,
    output tri [WORLD_SIZE-1:0] b_bus
);

  logic [WORLD_SIZE-1:0] registers[2**SEL_WIDTH-1];

  assign a_bus = oe_a ? registers[sel_a] : 'hz;
  assign b_bus = oe_b ? registers[sel_b] : 'hz;

  always_ff @(posedge clk) begin
    if (ld) registers[sel_a] <= input_bus;  // ignore sel_b
  end


endmodule
