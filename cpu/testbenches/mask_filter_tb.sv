module mask_filter_tb;
  import reg_pkg::*;

  wire [31:0] out;
  logic [31:0] in;
  reg_mask_e mask;

  mask_filter mask_filter0 (.*);
  initial begin
    #600;
    $display("\ntesting filter");
    test_filter('hffffffff, reg_pkg::LS8, 'h000000ff);
    test_filter('hfffff1f2, reg_pkg::LS16, 'h0000f1f2);
    test_filter('hff1ff1f2, reg_pkg::LS27, 'h001ff1f2);
    test_filter('h5f1ff1f2, reg_pkg::LS32, 'h5f1ff1f2);
  end

  task static test_filter(input logic [31:0] in_in, input reg_mask_e mask_in,
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
