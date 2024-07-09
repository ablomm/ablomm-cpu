module soc (
    input clk
);
  tri [31:0] a_bus, b_bus, result_bus;
  wire mem_rd, mem_wr;
  wire start;

  cpu cpu0 (
      .clk(clk),
	  .start(start),
      .a_bus(a_bus),
      .b_bus(b_bus),
      .result_bus(result_bus),
      .mem_rd(mem_rd),
      .mem_wr(mem_wr)
  );

  mem mem0 (
      .clk (clk),
      .rd  (mem_rd),
      .wr  (mem_wr),
      .addr(b_bus[15:0]),
      .data(a_bus),
      .out (result_bus)
  );

endmodule
