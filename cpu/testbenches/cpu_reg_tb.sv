module cpu_reg_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a;
  tri [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;
  logic [31:0] value;

  cpu_reg reg0 (.*);

  initial begin
    #100;
    $display("\ntesting cpu_reg");
    test_ld_oe(123);
    test_ld_oe(321);
  end

  task static test_ld_a(input logic [31:0] data_in);
    begin
      clk  = 0;
      oe_a = 0;
      oe_b = 0;
      #1;
      in  = data_in;
      ld  = 1;
      clk = 1;
      #1;
      ld = 0;
    end
  endtask

  task static test_oe_a;
    begin
      oe_a = 1;
      #1;
    end
  endtask

  task static test_oe_b;
    begin
      oe_b = 1;
      #1;
    end
  endtask

  task static test_ld_oe(input logic [31:0] data_in);
    begin
      test_ld_a(data_in);
      test_oe_a;
      $display("a = %d", a);
      assert (a === data_in)
      else $fatal;

      test_oe_b;
      $display("b_bus = %d", b);
      assert (b === data_in)
      else $fatal;
    end
  endtask
endmodule
