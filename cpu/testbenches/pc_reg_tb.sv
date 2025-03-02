module pc_reg_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a, b;
  logic [31:0] in;
  logic oe_a, oe_b, ld;
  logic post_inc = 0;
  logic [31:0] value;

  pc_reg pc (.*);

  initial begin
    #900;
    $display("\ntesting pc_reg");
    test_ld_oe(123);
    test_post_inc();
    test_ld_oe(321);
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

  task static test_post_inc();
    begin
      logic [31:0] pre_value;
      pre_value = value;

      clk = 0;
      post_inc = 1;
      #1;

      $display("post_inc before clk: value = %d, expected = %d", value, pre_value);
      assert (pre_value === value)
      else $fatal;

      clk = 1;
      #1;

      post_inc = 0;
      #1;

      $display("post_inc after clk: value = %d, expected = %d", value, pre_value + 1);
      assert (pre_value + 1 === value)
      else $fatal;
    end
  endtask
endmodule
