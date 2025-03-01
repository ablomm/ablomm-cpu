import timer_pkg::*;

module timer #(
    parameter integer WORD_SIZE = 32
) (
    input clk,
    input [WORD_SIZE-1:0] data,
    input timer_reg_e reg_sel,
    output tri [WORD_SIZE-1:0] out,
    input rd,
    input wr,
    output logic timeout = 0
);
  timer_ctrl_t control_reg;  // bit 0, start bit, bit 1 continue bit
  logic [WORD_SIZE-1:0] interval_reg;

  logic [WORD_SIZE-1:0] timer_reg;

  // reads
  assign out = rd ? sel_reg_val(reg_sel, control_reg, interval_reg, timer_reg) : 'hz;

  always_ff @(posedge clk) begin
    // decrement
    if (control_reg._start) begin
      timer_reg <= timer_reg - 1;

      // need to check timer_reg is 0 incase the timer register started at 0
      if (timer_reg === 'b0 || timer_reg - 1 === 'b0) begin
        timeout <= 1;

        // if continue is set, start again, else set stop (start = 0)
        if (control_reg._continue) timer_reg <= interval_reg;
        else control_reg[0] <= 0;
      end
    end

    // writes
    if (wr) begin
      unique case (reg_sel)
        timer_pkg::ACK: timeout <= 'b0;
        timer_pkg::CTRL: begin
          // checks for posedge of start bit
          // need to do this instead of `always @(posedge control_reg._start) timer_reg <= interval_reg;`
          // because apparently it's not good to drive the same values in
          // different sequential blocks
          if (control_reg._start !== 'b1 && data[0] === 'b1) timer_reg <= interval_reg;

          control_reg <= timer_ctrl_t'(data);
        end
        timer_pkg::INTERVAL: interval_reg <= data;
        timer_pkg::TIMER: timer_reg <= data;
        default: ;
      endcase
    end
  end

  // just a function to get the selected register val to assign to the output
  function static logic [WORD_SIZE-1:0] sel_reg_val(
      input timer_reg_e reg_sel, input timer_ctrl_t control_reg,
      input logic [WORD_SIZE-1:0] interval_reg, input logic [WORD_SIZE-1:0] timer_reg);
    unique case (reg_sel)
      timer_pkg::ACK: sel_reg_val = 'b0;
      timer_pkg::CTRL: sel_reg_val = WORD_SIZE'(control_reg);
      timer_pkg::INTERVAL: sel_reg_val = interval_reg;
      timer_pkg::TIMER: sel_reg_val = timer_reg;
      default: ;
    endcase
  endfunction
endmodule
