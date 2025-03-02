module lr_reg_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a, b;
  logic [31:0] in;
  logic oe_a, oe_b, ld;
  logic [31:0] pc;
  logic ld_pc;
  logic [31:0] value;

  lr_reg lr (.*);

  initial begin
    #500;
    $display("\ntesting lr_reg");
    test_ld_oe(123);
    test_ld_oe(321);
    test_ld_pc_oe(123);
    test_ld_pc_oe(321);
  end

  task static load(input logic [31:0] data_in);
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
      #1;
    end
  endtask

  task static load_pc(input logic [31:0] data_in);
    begin
      clk  = 0;
      oe_a = 0;
      oe_b = 0;
      #1;

      pc = data_in;
      ld_pc = 1;
      clk = 1;
      #1;

      ld = 0;
      #1;
    end
  endtask

  task static get_from_a(output logic [31:0] value_out);
    begin
      oe_a = 1;
      #1;

      value_out = value;
      oe_a = 0;
      #1;
    end
  endtask

  task static get_from_b(output logic [31:0] value_out);
    begin
      oe_b = 1;
      #1;

      value_out = value;
      oe_b = 0;
      #1;
    end
  endtask

  task static test_ld_oe(input logic [31:0] data_in);
    begin
      logic [31:0] read_value;

      load(data_in);

      get_from_a(read_value);
      $display("ld oe a: a = %d, expected = %d", read_value, data_in);
      assert (read_value === data_in)
      else $fatal;

      get_from_b(read_value);
      $display("ld oe b: b = %d, expected = %d", read_value, data_in);
      assert (read_value === data_in)
      else $fatal;
    end
  endtask

  task static test_ld_pc_oe(input logic [31:0] data_in);
    begin
      load_pc(data_in);
      oe_a = 1;
      oe_b = 1;
      #1;

      $display("ld pc oe a: a = %d, expected = %d", a, data_in);
      assert (a === data_in)
      else $fatal;

      $display("ld pc oe b: b = %d, expected = %d", b, data_in);
      assert (b === data_in)
      else $fatal;

      oe_a = 0;
      oe_b = 0;
      #1;
    end
  endtask
endmodule
