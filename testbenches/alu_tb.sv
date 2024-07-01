module alu_tb;

  `include "include/alu_op.vh"

  logic oe;
  logic [31:0] a, b;
  wire  [ 2:0] status;
  logic [ 3:0] operation;
  wire  [31:0] out;

  logic [31:0] c;

  alu a1 (
      .oe(oe),
      .a(a),
      .b(b),
      .out(out),
      .status(status),
      .operation(operation),
      .carry_in(1'b0)
  );

  initial begin
    $display("testing addition:");

    test_sum(1, 1, 3'b000);
    test_sum(2, 1, 3'b000);
    test_sum(32'hffffffff, 2, 3'b001);

    $display("\ntesting subtraction:");

    test_sub(1, 1, 3'b010);
    test_sub(2, 1, 3'b000);
    test_sub(2, 3, 3'b101);

    // $finish(0);
  end

  initial $monitor("a = %d, b = %d, out = %d, op = %h, status = %b", a, b, out, operation, status);

  task static test_sum(input logic [31:0] a_in, input logic [31:0] b_in,
                       input logic [2:0] status_in);
    begin
      operation = `ALU_ADD;
      a = a_in;
      b = b_in;
      oe = 1;
      #1;
      if (out !== a_in + b_in) $finish(1);
      if (status !== status_in) $finish(2);
      oe = 0;
    end
  endtask

  task static test_sub(input logic [31:0] a_in, input logic [31:0] b_in,
                       input logic [2:0] status_in);
    begin
      operation = `ALU_SUB;
      a = a_in;
      b = b_in;
      oe = 1;
      #1;
      if (out !== a_in - b_in) $finish(1);
      if (status !== status_in) $finish(2);
      oe = 0;
    end
  endtask

endmodule
