module mem_tb;
  logic clk;
  logic [31:0] data;
  logic [15:0] addr;
  tri [31:0] out;
  logic rd, wr, en = 1;

  mem m0 (.*);

  initial begin
    #700;
    $display("\ntesting mem");
    test_rd_wr(15, 123);
    test_rd_wr(16, 223);
  end

  task static write(input logic [15:0] addr_in, input logic [31:0] data_in);
    begin
      clk = 0;
      #1;

      addr = addr_in;
      data = data_in;
      wr   = 1;
      clk  = 1;
      #1;

      wr = 0;
      #1;
    end
  endtask

  task static read(input logic [15:0] addr_in, output logic [31:0] value_out);
    begin
      addr = addr_in;
      rd   = 1;
      #1;

      value_out = out;
      rd = 0;
      #1;
    end
  endtask

  task static test_rd_wr(input logic [15:0] addr_in, input logic [31:0] data_in);
    begin
      logic [31:0] read_value;

      write(addr_in, data_in);
      read(addr_in, read_value);

      $display("data = %d", read_value);
      assert (read_value === data_in)
      else $fatal;
    end
  endtask
endmodule
