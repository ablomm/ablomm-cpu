module cpu_reg_tb;
  logic clk;
  tri [31:0] a;
  tri [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;

  cpu_reg reg0 (
      .clk(clk),
      .a(a),
      .b(b),
      .in(in),
      .oe_a(oe_a),
      .oe_b(oe_b),
      .ld(ld)
  );

  initial begin
    test_ld_oe(123);
    test_ld_oe(321);
  end

  task static ld_a(input logic [31:0] data_in);
    begin
      clk = 0;
      oe_a = 0;
      oe_b = 0;
      #1;
      in = data_in;
      ld = 1;
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

  task static test_ld_oe(input logic [32:0] data_in);
    begin
      ld_a(data_in);
      set_oe_a();
      $display("a = %d", a);
      if (a !== data_in) $finish(1);

      set_oe_b();
      $display("b_bus = %d", b);
      if (b !== data_in) $finish(1);
    end
  endtask
endmodule
