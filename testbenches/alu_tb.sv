module alu_tb;
  import alu_pkg::*;

  logic oe;
  logic [31:0] a, b;
  wire [3:0] status;
  alu_op_e operation;
  wire [31:0] out;

  logic [31:0] c;

  alu alu0 (
	  .*,
      .carry_in(1'b0)
  );

  initial begin
    test_sum(1, 1, 4'b0000);
    test_sum(2, 1, 4'b0000);
    test_sum(32'hffffffff, 2, 4'b0010);
    test_sum(32'h7fffffff, 1, 4'b1001);

    test_sub(1, 1, 4'b0100);
    test_sub(2, 1, 4'b0000);
    test_sub(2, 3, 4'b1010);
    test_sub(-32'h80000000, 1, 4'b0001);
  end

  initial $monitor("a = %d, b = %d, out = %d, op = %h, status = %b", a, b, out, operation, status);

  task static test_sum(input logic [31:0] a_in, input logic [31:0] b_in,
                       input logic [3:0] status_in);
    begin
      operation = ADD;
      a = a_in;
      b = b_in;
      oe = 1;
      #1;
      assert (out === a_in + b_in);
      assert (status === status_in);
      oe = 0;
    end
  endtask

  task static test_sub(input logic [31:0] a_in, input logic [31:0] b_in,
                       input logic [3:0] status_in);
    begin
      operation = SUB;
      a = a_in;
      b = b_in;
      oe = 1;
      #1;
      assert (out === a_in - b_in);
      assert (status === status_in);
      oe = 0;
    end
  endtask

endmodule
