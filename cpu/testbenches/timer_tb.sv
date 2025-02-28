import timer_pkg::*;

module timer_tb;
  logic clk;
  logic [31:0] data;
  wire [31:0] out;
  timer_reg_e reg_sel;
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
  end

  task static load_interval(input logic [31:0] interval);
    begin
      clk = 0;
      data = interval;
      reg_sel = timer_pkg::INTERVAL;
      wr = 1;
      #1;
      clk = 1;
      #1;
      wr  = 0;
      clk = 0;
      #1;
      test_read(timer_pkg::INTERVAL, interval);
    end
  endtask

  task static start_timer();
    begin
      timer_ctrl_t timer_ctrl;
      // for some reason icarus verilog doesn't work with the '{_start: 1, default: 0} stynax
      timer_ctrl._start = 1;
      timer_ctrl._continue = 0;

      clk = 0;
      reg_sel = timer_pkg::CTRL;
      data = timer_ctrl;
      wr = 1;
      #1;
      clk = 1;
      #1;
      wr  = 0;
      clk = 0;
      #1;
      test_read(timer_pkg::CTRL, timer_ctrl);
    end
  endtask

  task static ack_interupt();
    begin
      clk = 0;
      reg_sel = timer_pkg::ACK;
      wr = 1;
      #1;
      clk = 1;
      #1;
      wr  = 0;
      clk = 0;
      #1;
    end
  endtask

  task static clock_timer();
    begin
      clk = 0;
      #1;
      clk = 1;
      #1;
      clk = 0;
      #1;
    end
  endtask

  task static test_read(timer_reg_e in_reg, logic [31:0] expected);
    begin
      reg_sel = in_reg;
      rd = 1;
      wr = 0;
      #1;

      $display("reading register 0b%b = %d; expected = %d", in_reg, out, expected);
      assert (out === expected)
      else $fatal;

      #1;
      rd = 0;
    end
  endtask

  task static test_interval(input logic [31:0] interval);
    $display("interval = %d", interval);
    begin
      clk = 0;
      load_interval(interval);

      start_timer();

      for (i = 1; i <= interval; i++) begin
        assert (timeout === 'b0)
        else $fatal;
        clock_timer();
        test_read(timer_pkg::TIMER, interval - i);
      end

      assert (timeout === 'b1)
      else $fatal;
      ack_interupt();
      assert (timeout === 'b0)
      else $fatal;
    end
  endtask
endmodule
