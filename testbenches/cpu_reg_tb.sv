module cpu_reg_tb;
  logic clk;
  tri [31:0] data_bus;
  tri [31:0] addr_bus;
  logic [31:0] result_bus;
  logic oe_data;
  logic oe_addr;
  logic ld;

  cpu_reg reg0 (
      .clk(clk),
      .data_bus(data_bus),
      .addr_bus(addr_bus),
      .result_bus(result_bus),
      .oe_data(oe_data),
      .oe_addr(oe_addr),
      .ld(ld)
  );

  initial begin
    test_ld_oe(123);
    test_ld_oe(321);
  end

  task static ld_data(input logic [31:0] data_in);
    begin
      clk = 0;
      oe_data = 0;
      oe_addr = 0;
      #1;
      result_bus = data_in;
      ld = 1;
      clk = 1;
      #1;
      ld = 0;
    end
  endtask

  task static set_oe_data();
    begin
      oe_data = 1;
      #1;
    end
  endtask

  task static set_oe_addr();
    begin
      oe_addr = 1;
      #1;
    end
  endtask

  task static test_ld_oe(input logic [32:0] data_in);
    begin
      ld_data(data_in);
      set_oe_data();
      $display("data_bus = %d", data_bus);
      if (data_bus !== data_in) $finish(1);

      set_oe_addr();
      $display("addr_bus = %d", addr_bus);
      if (addr_bus !== data_in) $finish(1);
    end
  endtask
endmodule
