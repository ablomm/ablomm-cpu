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
  logic post_inc_sp;
  logic pre_dec_sp;
  logic post_inc_pc;

  register_file reg_file0 (.*);

  initial begin
    test_ld_oe(reg_pkg::R0, 123);
    test_ld_oe(reg_pkg::R1, 321);
    test_ld_oe(reg_pkg::R0, 567);
  end

  task static ld_data(input reg_e sel_reg, input logic [31:0] data_in);
    begin
      clk = 0;
      #1;
      sel_in = sel_reg;
      in = data_in;
      ld = 1;
      clk = 1;
      #1;
      ld = 0;
    end
  endtask

  task static set_oe_a(input reg_e sel_reg);
    begin
      sel_a = sel_reg;
      oe_a  = 1;
      #1;
    end
  endtask

  task static set_oe_b(input reg_e sel_reg);
    begin
      sel_b = sel_reg;
      oe_b  = 1;
      #1;
    end
  endtask

  task static test_ld_oe(input reg_e sel_reg, input logic [32:0] data_in);
    begin
      ld_data(sel_reg, data_in);
      set_oe_a(sel_reg);
      $display("a = %d", a);
      assert (a === data_in);

      set_oe_b(sel_reg);
      $display("b = %d", b);
      assert (b === data_in);
    end
  endtask
endmodule
