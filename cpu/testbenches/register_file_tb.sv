import reg_pkg::*;
module register_file_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a, b;
  logic [31:0] in;
  logic oe_a, oe_b, ld;
  reg_e sel_a, sel_b, sel_in;

  register_file reg_file0 (.*);

  initial begin
    #1000;
    $display("\ntesting register_file");
    test_ld_oe(reg_pkg::R0, 123);
    test_ld_oe(reg_pkg::R1, 321);
    test_ld_oe(reg_pkg::R0, 567);
  end

  task static load(input reg_e sel_reg_in, input logic [31:0] data_in);
    begin
      clk = 0;
      #1;

      sel_in = sel_reg_in;
      in = data_in;
      ld = 1;
      clk = 1;
      #1;

      ld = 0;
      #1;
    end
  endtask

  task static get_from_a(input reg_e sel_reg_in, output [31:0] value_out);
    begin
      sel_a = sel_reg_in;
      oe_a  = 1;
      #1;

      value_out = a;
      oe_a = 0;
      #1;
    end
  endtask

  task static get_from_b(input reg_e sel_reg_in, output [31:0] value_out);
    begin
      sel_b = sel_reg_in;
      oe_b  = 1;
      #1;

      value_out = b;
      oe_b = 0;
      #1;
    end
  endtask

  task static test_ld_oe(input reg_e sel_reg_in, input logic [31:0] data_in);
    begin
      logic [31:0] read_value;

      load(sel_reg_in, data_in);

      get_from_a(sel_reg_in, read_value);
      $display("ld oe a: a = %d, expected = %d", read_value, data_in);
      assert (read_value === data_in)
      else $fatal;

      get_from_b(sel_reg_in, read_value);
      $display("ld oe b: b = %d, expected = %d", read_value, data_in);
      assert (read_value === data_in)
      else $fatal;
    end
  endtask
endmodule
