module cpu_reg #(
    parameter integer SIZE = 32
) (
    input clk,
    output tri [SIZE-1:0] data_bus,
    output tri [SIZE-1:0] addr_bus,
	input [SIZE-1:0] result_bus,
    input oe_data,
    input oe_addr,
    input ld,
    output logic [SIZE-1:0] value  // only if you need to direclty access (not on the data/addr bus)
);

  assign out  = value;
  assign data_bus = oe_data ? value : 'hz;
  assign addr_bus = oe_addr ? value : 'hz;

  always_ff @(posedge clk) begin
    if (ld) value <= result_bus;
  end

endmodule
