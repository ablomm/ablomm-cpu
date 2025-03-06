module full_adder_tb;
  logic [31:0] a, b;
  wire [31:0] out;
  wire carry_out, overflow;

  full_adder full_adder (.*);

  initial begin
    #400;
    $display("\ntesting full_adder");
    test_adder(1, 1, 2, 0, 0);  // 1 + 1
    test_adder(2, 1, 3, 0, 0);  // 2 + 1
    test_adder(32'hffffffff, 2, 1, 1, 0);  // -1 + 2
    test_adder(32'h7fffffff, 1, 32'h80000000, 0, 1);  // 2^31-1 + 1 (signed overflow)
  end


  task static test_adder(logic [31:0] in_a, logic [31:0] in_b, logic [31:0] expected_out,
                         logic expected_carry_out, logic expected_overflow);
    begin
      a = in_a;
      b = in_b;
      #1;

      $display(
          "a = %d, b = %d, carry_out = %d, overflow = %d, expected_out = %d, expected_carry_out = %d, expected_overflow = %d",
          a, b, out, carry_out, overflow, expected_out, expected_carry_out, expected_overflow);

      assert (out === expected_out)
      else $fatal;

      assert (carry_out === expected_carry_out)
      else $fatal;

      assert (expected_overflow === expected_overflow)
      else $fatal;
    end

  endtask
endmodule
