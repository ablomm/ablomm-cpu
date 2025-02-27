module power (
    input clk,
    input [1:0] data,
    input wr,
    input en,
    output logic rst
);
  `define SHUTDOWN 0
  `define RESTART 1

  always @(posedge clk) begin
    if (en && wr) begin

      if (data === `SHUTDOWN) $finish;

      else if (data === `RESTART) begin
        rst = 1;
        #10;
        rst = 0;
      end

    end
  end
endmodule
