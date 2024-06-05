module counter #(
    parameter integer SIZE = 32
) (
    input clk,
    input rst,
    input cnt,
    input ld,
    input oe,
    input [SIZE-1:0] in,
    output tri [SIZE-1:0] out
);

  reg [SIZE-1:0] out_reg;

  assign out = oe ? out_reg : 'hz;

  always_ff @(posedge clk) begin
    if (cnt) out_reg <= out_reg + 1;
	if (ld) out_reg <= in;
  end

  always @(posedge rst) out_reg <= 0;

endmodule
