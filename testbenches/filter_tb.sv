module filter_tb;
  wire  [31:0] out;
  logic [31:0] in;
  logic [31:0] mask;

  filter filter0 (.*);
  initial begin
    test_filter(32'hffffffff, 32'hf0f0f0f0);
    test_filter(32'h12312312, 32'h50f37431);
  end

  task static test_filter(input logic [31:0] in_in, input logic [31:0] mask_in);
    begin
      in   = in_in;
      mask = mask_in;
      #1;
      $display("out = %d", out);
      assert (out === (in_in & mask_in));
    end
  endtask
endmodule
