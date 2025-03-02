module offset_filter_tb;
  wire  [31:0] out;
  logic [31:0] in;
  logic [11:0] offset;

  offset_filter #(.OFFSET_WIDTH(12)) offset_filter (.*);
  initial begin
    #800;
    $display("\ntesting offset filter");
    test_filter('h0, 'h123, 'h123);
    test_filter('hffffffff, 'h1, 'h0);
    test_filter('h12312312, 'h431, 'h12312743);
    test_filter('h12345678, 'hfff, 'h12345677);  // negative 1
  end

  task static test_filter(input logic [31:0] in_in, input logic [11:0] offset_in,
                          input logic [31:0] expected);
    begin
      in = in_in;
      offset = offset_in;
      #1;

      $display("in = %h, offset = %h, out = %h, expected = %h", in_in, offset_in, out, expected);
      assert (out === expected)
      else $fatal;
    end
  endtask
endmodule
