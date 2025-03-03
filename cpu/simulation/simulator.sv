module simulator;
  logic clk = 0;
  wire irq;  // set by ic
  wire rst;  // set by power controller
  logic start = 1;
  wire [31:0] addr;
  tri [31:0] data;
  wire rd, wr;

  initial forever #10 clk = ~clk;

  cpu cpu0 (.*);

  // 0x0000 to 0x3fff (2^14 addresses)
  rom #(
      .ADDR_WIDTH(14)
  ) rom0 (
      .clk (clk),
      .addr(addr[13:0]),
      .out (data),
      .rd  (rd),
      .en  (addr[15:14] === 2'b00)
  );

  wire timer_int;

  // 0x4000 to 0x4003 (4 addresses)
  timer timer0 (
      .clk(clk),
      .data(data),
      .reg_sel(addr[1:0]),
      .out(data),
      .rd(rd && addr[15:2] === 14'h1000),
      .wr(wr && addr[15:2] === 14'h1000),
      .timeout(timer_int)
  );

  // 0x4004
  wire [15:0] irq_sources = {{15{1'b0}}, timer_int};
  ic ic0 (
      .clk(clk),
      .irq_in(irq_sources),
      .out(data[15:0]),
      .rd(rd && addr[15:0] === 16'h4004),
      .irq_out(irq)
  );

  // memory mapped power controller for simulation
  // 0x4005
  power power0 (
      .clk (clk),
      .data(data[1:0]),
      .wr  (wr),
      .en  (addr[15:0] === 16'h4005),
      .rst (rst)
  );

  // memory mapped terminal for simulation
  // 0x4006
  tty tty0 (
      .clk (clk),
      .data(data[7:0]),
      .wr  (wr),
      .en  (addr[15:0] === 16'h4006)
  );

  // 0x8000 to 0xffff (2^15 addresses)
  mem #(
      .ADDR_WIDTH(15)
  ) mem0 (
      .clk (clk),
      .addr(addr[14:0]),
      .data(data),
      .out (data),
      .rd  (rd),
      .wr  (wr),
      .en  (addr[15] === 1'b1)
  );

endmodule
