module simulator;
  logic clk = 0;
  wire  hwint;  // set by pic
  wire  rst;  // set by power controller
  logic start = 1;
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
      .out (result_bus),
      .rd  (mem_rd),
      .en  (b_bus[15:14] === 2'b00)
  );

  wire timer_int;
  timer timer0 (
      .clk(clk),
      .data(a_bus),
      .reg_sel(b_bus[1:0]),
      .out(result_bus),
      .rd(mem_rd && b_bus[15:2] === 14'h1000),
      .wr(mem_wr && b_bus[15:2] === 14'h1000),
      .timeout(timer_int)
  );

  wire [15:0] irq_sources = {{15{1'b0}}, timer_int};

  ic ic0 (
      .clk (clk),
      .irq (irq_sources),
      .out (result_bus),
      .rd  (mem_rd && b_bus[15:0] === 16'h4004),
      .intr(hwint)
  );

  // memory mapped power controller for simulation
  power power0 (
      .clk (clk),
      .data(a_bus[1:0]),
      .wr  (mem_wr),
      .en  (b_bus[15:0] === 16'h4005),
      .rst (rst)
  );

  // memory mapped terminal for simulation
  tty tty0 (
      .clk (clk),
      .data(a_bus[7:0]),
      .wr  (mem_wr),
      .en  (b_bus[15:0] === 16'h4006)
  );
endmodule
