module simulator;
  logic clk = 0;
  logic start = 1, rst = 0;
  tri [31:0] a_bus, b_bus, result_bus;
  wire mem_rd, mem_wr;

  initial forever #10 clk = ~clk;

  cpu cpu0 (.*);

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
