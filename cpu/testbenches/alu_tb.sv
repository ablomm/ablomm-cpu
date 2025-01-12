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
    carry_in = 0;
    #1;
    $display("\ntesting alu");
    $display("\nPASS:");
    test_alu(alu_pkg::PASS, 0, 5, 5, 4'b0000);  // 5
    test_alu(alu_pkg::PASS, 0, -123, -123, 4'b1000);  // -123
    test_alu(alu_pkg::PASS, 0, 0, 0, 4'b0100);  // 0

    $display("\nAND:");
    test_alu(alu_pkg::AND, 'b101, 'b010, 'b000, 4'b0100);  // 0b101 & 0b010
    test_alu(alu_pkg::AND, 'b101, 'b111, 'b101, 4'b0000);  // 0b101 & 0b111

    $display("\nOR:");
    test_alu(alu_pkg::OR, 'b111, 'b010, 'b111, 4'b0000);  // 0b101 | 0b010
    test_alu(alu_pkg::OR, 'b101, 'b000, 'b101, 4'b0000);  // 0b101 | 0b000

    $display("\nXOR:");
    test_alu(alu_pkg::XOR, 'b111, 'b010, 'b101, 4'b0000);  // 0b111 ^ 0b010
    test_alu(alu_pkg::XOR, 'b110, 'b101, 'b011, 4'b0000);  // 0b110 ^ 0b101

    $display("\nNOT:");
    test_alu(alu_pkg::NOT, 0, 'hffffffff, 0, 4'b0100);  // ~0xffffffff
    test_alu(alu_pkg::NOT, 0, 'h80000000, 'h7fffffff, 4'b0000);  // ~0x80000000

    $display("\nADD:");
    test_alu(alu_pkg::ADD, 1, 1, 2, 4'b0000);  // 1 + 1
    test_alu(alu_pkg::ADD, 2, 1, 3, 4'b0000);  // 2 + 1
    test_alu(alu_pkg::ADD, 32'hffffffff, 2, 1, 4'b0010);  // i.e. -1 + 2
    test_alu(alu_pkg::ADD, 32'h7fffffff, 1, 32'h80000000, 4'b1001);  // 2^31-1 + 1 (signed overflow)

    $display("\nADDC:");
    carry_in = 1;
    test_alu(alu_pkg::ADDC, 1, 1, 3, 4'b0000);  // 1 + 1 + 1
    test_alu(alu_pkg::ADDC, 2, 1, 4, 4'b0000);  // 2 + 1 + 1
    test_alu(alu_pkg::ADDC, 32'hffffffff, 2, 2, 4'b0010);  // -1 + 2 + 1
    test_alu(alu_pkg::ADDC, 32'h7fffffff, 1, 32'h80000001, 4'b1001);  // 2^31-1 + 1 + 1
    carry_in = 0;
    test_alu(alu_pkg::ADDC, 1, 1, 2, 4'b0000);  // 1 + 1 + 0
    test_alu(alu_pkg::ADDC, 2, 1, 3, 4'b0000);  // 2 + 1 + 0
    test_alu(alu_pkg::ADDC, 32'hffffffff, 2, 1, 4'b0010);  // -1 + 2 + 0
    test_alu(alu_pkg::ADDC, 32'h7fffffff, 1, 32'h80000000, 4'b1001);  // 2^31-1 + 1 + 0

    $display("\nSUB:");
    test_alu(alu_pkg::SUB, 1, 1, 0, 4'b0110);  // 1 - 1
    test_alu(alu_pkg::SUB, 2, 1, 1, 4'b0010);  // 2 - 1
    test_alu(alu_pkg::SUB, 2, 3, -1, 4'b1000);  // 2 - 3
    test_alu(alu_pkg::SUB, 32'h80000000, 1, 32'h7fffffff, 4'b0011);  // 2^32 - 1 (signed underflow)

    $display("\nSUBB:");
    carry_in = 0;
    test_alu(alu_pkg::SUBB, 5, 1, 3, 4'b0010);  // 5 - 1 - 1
    test_alu(alu_pkg::SUBB, 1, 1, -1, 4'b1000);  // 1 - 1 - 1
    test_alu(alu_pkg::SUBB, 2, 1, 0, 4'b0110);  // 2 - 1 - 1
    test_alu(alu_pkg::SUBB, 2, 3, -2, 4'b1000);  // 2 - 3 - 1
    test_alu(alu_pkg::SUBB, 32'h80000000, 1, 32'h7ffffffe, 4'b0011);  // 2^32 - 1 - 1
    carry_in = 1;
    test_alu(alu_pkg::SUBB, 5, 1, 4, 4'b0010);  // 5 - 1
    test_alu(alu_pkg::SUBB, 1, 1, 0, 4'b0110);  // 1 - 1
    test_alu(alu_pkg::SUBB, 2, 1, 1, 4'b0010);  // 2 - 1
    test_alu(alu_pkg::SUBB, 2, 3, -1, 4'b1000);  // 2 - 3
    test_alu(alu_pkg::SUBB, 32'h80000000, 1, 32'h7fffffff, 4'b0011);  // 2^32 - 1

    $display("\nNEG:");
    test_alu(alu_pkg::NEG, 0, 0, 0, 4'b0100);  // -0
    test_alu(alu_pkg::NEG, 0, 1, -1, 4'b1000);  // -1
    test_alu(alu_pkg::NEG, 0, 123, -123, 4'b1000);  // -123
    test_alu(alu_pkg::NEG, 0, -123, 123, 4'b0000);  // --123

    $display("\nSHL:");
    test_alu(alu_pkg::SHL, 1, 1, 'b10, 4'b0000);  // 1 << 1
    test_alu(alu_pkg::SHL, 1, 5, 'b100000, 4'b0000);  // 1 << 5
    test_alu(alu_pkg::SHL, 'b101, 4, 'b1010000, 4'b0000);  // 0b101 << 4
    test_alu(alu_pkg::SHL, 32'h80000000, 1, 0, 4'b0110);  // 2^31 << 1 (off the side)

    $display("\nSHR:");
    test_alu(alu_pkg::SHR, 'b100, 1, 'b10, 4'b0000);  // 0b100 >> 1
    test_alu(alu_pkg::SHR, 1, 1, 0, 4'b0100);  // 1 >> 1
    test_alu(alu_pkg::SHR, 'b1010000, 4, 'b101, 4'b0000);  // 0b1010000 >> 4

    $display("\nASHR:");
    test_alu(alu_pkg::ASHR, -'b100, 1, -'b10, 4'b1000);  // -0b100 >>> 1
    test_alu(alu_pkg::ASHR, -'b100, 3, -1, 4'b1000);  // -0b100 >>> 3
    test_alu(alu_pkg::ASHR, 'b100, 1, 'b10, 4'b0000);  // 0b100 >>> 1
    test_alu(alu_pkg::ASHR, 'b100, 3, 0, 4'b0100);  // 0b100 >>> 3
  end

  task static test_alu(input alu_op_e operation_in, input logic [31:0] a_in,
                       input logic [31:0] b_in, input logic [31:0] expected_result_in,
                       input alu_status_t expected_status_in);
    begin
      operation = operation_in;
      a = a_in;
      b = b_in;
      oe = 1;
      #1;

      $display("a = %d, b = %d, out = %d, op = %h, status = %b", a, b, out, operation, status);
      assert (out === expected_result_in)
      else $fatal;
      assert (status === expected_status_in)
      else $fatal;

      oe = 0;
      #1;
    end
  endtask
endmodule
