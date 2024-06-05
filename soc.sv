module soc (
    input clk
);
  tri [31:0] data_bus, addr_bus, result_bus;
  wire mem_rd, mem_wr;

  cpu cpu0 (
      .clk(clk),
      .data_bus(data_bus),
      .addr_bus(addr_bus),
	  .result_bus(result_bus),
      .mem_rd(mem_rd),
      .mem_wr(mem_wr)
  );

  mem mem0 (
	  .clk (clk),
      .rd  (mem_rd),
      .wr  (mem_wr),
      .addr(addr_bus[15:0]),
      .data(data_bus),
	  .out (result_bus)
  );
endmodule
