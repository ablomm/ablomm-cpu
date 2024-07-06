module register_file_tb;
  logic clk;
  tri [31:0] a;
  tri [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;
  logic [7:0] sel_a;
  logic [7:0] sel_b;

  register_file reg_file0 (
      .clk(clk),
      .a(a),
      .b(b),
      .in(in),
      .oe_a(oe_a),
      .oe_b(oe_b),
      .ld(ld),
      .sel_a(sel_a),
      .sel_b(sel_b)
  );

  initial begin
    test_ld_oe(2, 123);
    test_ld_oe(3, 321);
    test_ld_oe(2, 567);
  end

  task static ld_data(input logic [7:0] addr, input logic [31:0] data_in);
    begin
      clk  = 0;
      oe_a = 0;
      oe_b = 0;
      #1;
      sel_a = addr;
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

  task static test_ld_oe(input logic [7:0] addr, input logic [32:0] data_in);
    begin
      ld_data(addr, data_in);
      set_oe_a(addr);
      $display("a = %d", a);
      if (a !== data_in) $finish(1);

      set_oe_b(addr);
      $display("b = %d", b);
      if (b !== data_in) $finish(1);
    end
  endtask
endmodule
