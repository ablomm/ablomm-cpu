module timer_tb;
  logic clk;
  logic [31:0] data;
  logic [1:0] reg_sel;
  logic rd, wr;
  wire timeout;

  integer i;
  timer timer (.*);

  initial begin
    #700;
    $display("\ntesting timer");
    test_interval(4);
    test_interval(8);
    test_interval(10);
    test_interval(50);
  end

  task static load_interval(input logic [31:0] interval);
    begin
      clk = 0;
      data = interval;
      reg_sel = 'b10;
      wr = 1;
      #1;
      clk = 1;
      #1;
      wr  = 0;
      clk = 0;
      #1;
    end
  endtask

  task static start_timer();
    begin
      clk = 0;
      reg_sel = 'b01;
      data = 'b1;
      wr = 1;
      #1;
      clk = 1;
      #1;
      wr  = 0;
      clk = 0;
      #1;
    end
  endtask

  task static ack_interupt();
    begin
      clk = 0;
      reg_sel = 'b0;
      wr = 1;
      #1;
      clk = 1;
      #1;
      wr  = 0;
      clk = 0;
      #1;
    end
  endtask

  task static test_interval(input logic [31:0] interval);
    $display("interval = %d", interval);
    begin
      clk = 0;
      load_interval(interval);
      start_timer();

      for (i = 0; i <= interval - 1; i++) begin
        assert (timeout === 'b0);
        clk = 1;
        #1;
        clk = 0;
        #1;
      end

      assert (timeout === 'b1);
      ack_interupt();
      assert (timeout === 'b0);
    end
  endtask
endmodule
