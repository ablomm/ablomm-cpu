module mem_tb;
  logic clk;
  logic [31:0] data;
  logic [15:0] addr;
  tri [31:0] out;
  logic rd, wr, en;

  mem m0 (.*);

  initial begin
    #500;
    $display("\ntesting mem");
    en = 1;
    test_rd_wr(15, 123);
    test_rd_wr(16, 223);
  end

  task static test_write(input logic [15:0] addr_in, input logic [32:0] data_in);
    begin
      clk = 0;
      #1;
      addr = addr_in;
      data = data_in;
      rd   = 0;
      wr   = 1;
      clk  = 1;
      #1;
    end
  endtask

  task static test_read(input logic [15:0] addr_in);
    begin
      addr = addr_in;
      rd   = 1;
      wr   = 0;
      #1;
    end
  endtask

  task static test_rd_wr(input logic [15:0] addr_in, input logic [32:0] data_in);
    begin
      test_write(addr_in, data_in);
      test_read(addr_in);
      $display("data = %d", out);
      assert (out === data_in)
      else $fatal;
    end
  endtask
endmodule
