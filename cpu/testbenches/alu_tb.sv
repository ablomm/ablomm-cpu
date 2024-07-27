module alu_tb;
  import alu_pkg::*;

  logic oe;
  alu_op_e operation;
  logic carry_in;
  logic [31:0] a, b;
  wire [31:0] out;
  wire [ 3:0] status;

  alu alu0 (.*);

  initial begin
    #1;
    $display("\ntesting alu");
    test_alu(alu_pkg::ADD, 1, 1, 2, 4'b0000);
    test_alu(alu_pkg::ADD, 2, 1, 3, 4'b0000);
    test_alu(alu_pkg::ADD, 32'hffffffff, 2, 1, 4'b0010);
    test_alu(alu_pkg::ADD, 32'h7fffffff, 1, 32'h80000000, 4'b1001);

    test_alu(alu_pkg::SUB, 1, 1, 0, 4'b0100);
    test_alu(alu_pkg::SUB, 2, 1, 1, 4'b0000);
    test_alu(alu_pkg::SUB, 2, 3, -1, 4'b1010);
    test_alu(alu_pkg::SUB, 32'h80000000, 1, 32'h7fffffff, 4'b0001);
  end

  task static test_alu(input alu_op_e operation_in, input logic [31:0] a_in,
                       input logic [31:0] b_in, input [31:0] expected_result_in,
                       input alu_status_t expected_status_in);
    begin
      operation = operation_in;
      a = a_in;
      b = b_in;
      oe = 1;
      #1;

      $display("a = %d, b = %d, out = %d, op = %h, status = %b", a, b, out, operation, status);
      assert (out === expected_result_in);
      assert (status === expected_status_in);

      oe = 0;
      #1;
    end
  endtask
endmodule
