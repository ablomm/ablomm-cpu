module cpu_reg_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a;
  tri [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;
  logic [7:0] count;
  logic pre_count;
  logic post_count;
  logic [31:0] value;

  cpu_reg reg0 (.*);

  initial begin
    test_ld_oe(123);
    test_ld_oe(321);
    do_post_count(4);
    $display("a = %d", a);
    assert (a === 321 + 4);
    do_pre_count(-5);
    $display("a = %d", a);
    assert (a === 321 - 1);
  end

  task static ld_a(input logic [31:0] data_in);
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

  task static set_oe_a();
    begin
      oe_a = 1;
      #1;
    end
  endtask

  task static set_oe_b();
    begin
      oe_b = 1;
      #1;
    end
  endtask

  task static do_post_count(logic [7:0] count_in);
    begin
      clk = 0;
      count = count_in;
      post_count = 1;
      #1;
      clk = 1;
      #1;
      post_count = 0;
    end
  endtask

  task static do_pre_count(logic [7:0] count_in);
    begin
      clk = 0;
      count = count_in;
      pre_count = 1;
      #1;
      pre_count = 0;
    end
  endtask

  task static test_ld_oe(input logic [31:0] data_in);
    begin
      ld_a(data_in);
      set_oe_a();
      $display("a = %d", a);
      assert (a === data_in);

      set_oe_b();
      $display("b_bus = %d", b);
      assert (b === data_in);
    end
  endtask
endmodule
