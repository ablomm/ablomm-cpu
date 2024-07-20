module register_file_tb;
  logic clk;
  logic rst = 0;
  tri [31:0] a;
  tri [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;
  logic [7:0] sel_a;
  logic [7:0] sel_b;
  logic [7:0] sel_in;
  logic [7:0] count_a = 0;
  logic [7:0] count_b = 0;
  logic pre_count_a;
  logic pre_count_b;
  logic post_count_a;
  logic post_count_b;

  register_file reg_file0 (.*);

  initial begin
    test_ld_oe(2, 123);
    test_ld_oe(3, 321);
    test_ld_oe(2, 567);

    post_count(2, 5);
    set_oe_a(2);
    $display("a = %d", a);
    assert (a === 567 + 5);
    post_count(2, -6);
    $display("a = %d", a);
    assert (a === 567 - 1);
    pre_count(3, 2);
    $display("a = %d", a);
    assert (a === 321 + 2);
  end

  task static ld_data(input logic [7:0] addr, input logic [31:0] data_in);
    begin
      clk = 0;
      #1;
      sel_in = addr;
      in = data_in;
      ld = 1;
      clk = 1;
      #1;
      ld = 0;
    end
  endtask

  task static set_oe_a(input logic [7:0] addr);
    begin
      sel_a = addr;
      oe_a  = 1;
      #1;
    end
  endtask

  task static set_oe_b(input logic [7:0] addr);
    begin
      sel_b = addr;
      oe_b  = 1;
      #1;
    end
  endtask

  task static post_count(input logic [7:0] addr, input logic [7:0] count);
    begin
      clk = 0;
      sel_a = addr;
      count_a = count;
      post_count_a = 1;
      #1;
      clk = 1;
      post_count_a = 0;
      #1;
    end
  endtask

  task static pre_count(input logic [7:0] addr, input logic [7:0] count);
    begin
      clk = 0;
      #1;
      sel_a = addr;
      count_a = count;
      pre_count_a = 1;
      post_count_a <= 0;
      #1;
    end
  endtask

  task static test_ld_oe(input logic [7:0] addr, input logic [32:0] data_in);
    begin
      ld_data(addr, data_in);
      set_oe_a(addr);
      $display("a = %d", a);
      assert (a === data_in);

      set_oe_b(addr);
      $display("b = %d", b);
      assert (b === data_in);
    end
  endtask
endmodule
