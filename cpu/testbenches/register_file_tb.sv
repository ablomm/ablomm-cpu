import reg_pkg::*;
module register_file_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a;
  tri [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;
  reg_e sel_a;
  reg_e sel_b;
  reg_e sel_in;

  register_file reg_file0 (.*);

  initial begin
    #600;
    $display("\ntesting register_file");
    test_ld_oe(reg_pkg::R0, 123);
    test_ld_oe(reg_pkg::R1, 321);
    test_ld_oe(reg_pkg::R0, 567);
  end

  task static test_ld(input reg_e sel_reg_in, input logic [31:0] data_in);
    begin
      clk = 0;
      #1;
      sel_in = sel_reg_in;
      in = data_in;
      ld = 1;
      clk = 1;
      #1;
      ld = 0;
    end
  endtask

  task static test_oe_a(input reg_e sel_reg_in);
    begin
      sel_a = sel_reg_in;
      oe_a  = 1;
      #1;
    end
  endtask

  task static test_oe_b(input reg_e sel_reg_in);
    begin
      sel_b = sel_reg_in;
      oe_b  = 1;
      #1;
    end
  endtask

  task static test_ld_oe(input reg_e sel_reg_in, input logic [32:0] data_in);
    begin
      test_ld(sel_reg_in, data_in);
      test_oe_a(sel_reg_in);
      $display("a = %d", a);
      assert (a === data_in)
      else $fatal;

      test_oe_b(sel_reg_in);
      $display("b = %d", b);
      assert (b === data_in)
      else $fatal;
    end
  endtask
endmodule
