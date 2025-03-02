module mask_filter_tb;
  wire [31:0] out;
  logic [31:0] in, mask;

  mask_filter mask_filter0 (.*);
  initial begin
    #600;
    $display("\ntesting filter");
    test_filter('hffffffff, 'hf0f0f0f0, 'hf0f0f0f0);
    test_filter('h12312312, 'h50f37431, 'h10312010);
  end

  task static test_filter(input logic [31:0] in_in, input logic [31:0] mask_in,
                          input logic [31:0] expected);
    begin
      in   = in_in;
      mask = mask_in;
      #1;

      $display("in = %h, mask = %h, out = %h, expected = %h", in_in, mask_in, out, expected);
      assert (out === expected)
      else $fatal;
    end
  endtask
endmodule
