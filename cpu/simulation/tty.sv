module tty (
    input clk,
    input [7:0] data,
    input wr,
    input en
);

  always @(posedge clk) begin
    if (en && wr) $write("%s", data);
  end
endmodule
