module register_file_tb;
  logic clk;
  logic oe_a;
  logic oe_b;
  logic ld;
  logic [7:0] sel_a;
  logic [7:0] sel_b;
  logic [31:0] input_bus;
  tri [31:0] a_bus;
  tri [31:0] b_bus;

  register_file reg_file0 (
      .clk(clk),
      .oe_a(oe_a),
      .oe_b(oe_b),
      .ld(ld),
      .sel_a(sel_a),
      .sel_b(sel_b),
      .input_bus(input_bus),
      .a_bus(a_bus),
      .b_bus(b_bus)
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
      input_bus = data_in;
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
      $display("a_bus = %d", a_bus);
      if (a_bus !== data_in) $finish(1);

      set_oe_b(addr);
      $display("b_bus = %d", b_bus);
      if (b_bus !== data_in) $finish(1);
    end
  endtask
endmodule
