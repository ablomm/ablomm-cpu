import timer_pkg::*;

module timer_tb;
  logic clk;
  logic [31:0] data;
  wire [31:0] out;
  timer_reg_e reg_sel;
  logic rd, wr;
  wire timeout;

  timer timer (.*);

  initial begin
    #1300;
    $display("\ntesting timer");
    test_timer(4);
    test_timer(8);
    test_timer(10);
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
      data = 32'(timer_ctrl);
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

  task static read_register(input timer_reg_e reg_in, output logic [31:0] value_out);
    begin
      reg_sel = reg_in;
      rd = 1;
      wr = 0;
      #1;

      value_out = out;
      rd = 0;
      #1;
    end
  endtask

  task static test_read_expected(input timer_reg_e reg_in, input logic [31:0] expected);
    begin
      logic [31:0] read_value;

      read_register(reg_in, read_value);

      $display("reading register 0b%b = %d; expected = %d", reg_in, read_value, expected);
      assert (read_value === expected)
      else $fatal;
    end
  endtask

  task static test_timer(input logic [31:0] interval);
    begin
      integer i;

      $display("interval = %d", interval);
      load_interval(interval);
      test_read_expected(timer_pkg::INTERVAL, interval);

      start_timer();
      test_read_expected(timer_pkg::CTRL, 'b01);

      for (i = 1; i <= interval; i++) begin
        assert (timeout === 'b0)
        else $fatal;
        clock_timer();
        test_read_expected(timer_pkg::TIMER, interval - i);
      end

      assert (timeout === 'b1)
      else $fatal;
      ack_interupt();
      assert (timeout === 'b0)
      else $fatal;
    end
  endtask
endmodule
