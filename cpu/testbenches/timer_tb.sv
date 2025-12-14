module timer_tb;
  import timer_pkg::*;

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
    test_continue(10, 3);
  end

  task static load_reg(input timer_reg_e reg_in, input logic [31:0] data_in);
    begin
      clk = 0;
      data = data_in;
      reg_sel = reg_in;
      wr = 1;
      #1;

      clk = 1;
      #1;

      wr  = 0;
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

      $display("\ninterval = %d", interval);
      load_reg(timer_pkg::TIMER, interval);
      test_read_expected(timer_pkg::TIMER, interval);

      load_reg(timer_pkg::CTRL, 'b01);
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

      load_reg(timer_pkg::CTRL, 'b00);
    end
  endtask

  task static test_continue(input logic [31:0] interval, input integer continue_count);
    begin
      integer i;
      integer j;

      $display("\ncontinue interval = %d", interval);
      load_reg(timer_pkg::INTERVAL, interval);
      load_reg(timer_pkg::TIMER, interval);
      test_read_expected(timer_pkg::TIMER, interval);
      test_read_expected(timer_pkg::INTERVAL, interval);

      load_reg(timer_pkg::CTRL, 'b11);
      test_read_expected(timer_pkg::CTRL, 'b11);

      // need to clock before to be consistent with the continue case
      // which will have an interrupt when the timer is reset to interval.
      // This just makes it possible to use the for loop for when the timer
      // starts initially, and when it resets with the interval
      clock_timer();
      test_read_expected(timer_pkg::TIMER, interval - 1);

      // outer loop: number of times continued
      for (i = 0; i < continue_count; i++) begin
        // inner loop: counting down, -1 because the timer will reset at 0
        for (j = 2; j <= interval - 1; j++) begin
          assert (timeout === 'b0)
          else $fatal;
          clock_timer();
          test_read_expected(timer_pkg::TIMER, interval - j);
        end

        // clock once more to get last value
        clock_timer();
        test_read_expected(timer_pkg::TIMER, interval);
        assert (timeout === 'b1)
        else $fatal;

        // this takes one clock cycle, so the timer will be decremented
        ack_interupt();
        test_read_expected(timer_pkg::TIMER, interval - 1);
        assert (timeout === 'b0)
        else $fatal;
      end

      load_reg(timer_pkg::CTRL, 'b00);
    end
  endtask
endmodule
