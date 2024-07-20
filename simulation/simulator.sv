module simulator;
  logic clk = 0;
  logic start;
  tri [31:0] a_bus, b_bus, result_bus;
  wire mem_rd, mem_wr;

  initial forever #10 clk = ~clk;

  initial start = 1;

  cpu cpu0 (
      .clk(clk),
      .start(start),
      .a_bus(a_bus),
      .b_bus(b_bus),
      .result_bus(result_bus),
      .mem_rd(mem_rd),
      .mem_wr(mem_wr)
  );

  mem #(
      .ADDR_WIDTH(15)
  ) mem0 (
      .clk (clk),
      .addr(b_bus[14:0]),
      .data(a_bus),
      .out (result_bus),
      .rd  (mem_rd),
      .wr  (mem_wr),
      .en  (b_bus[15] === 1'b1)
  );

  // puts rom.txt and uses it as memory
  rom #(
      .ADDR_WIDTH(14)
  ) rom0 (
      .clk (clk),
      .addr(b_bus[13:0]),
      .data(a_bus),
      .out (result_bus),
      .rd  (mem_rd),
      .wr  (mem_wr),
      .en  (b_bus[15:14] === 2'b00)
  );

  // memory mapped terminal for simulation
  tty tty0 (
      .clk (clk),
      .data(a_bus[7:0]),
      .wr  (mem_wr),
      .en  (b_bus === 16'h4000)
  );
endmodule
