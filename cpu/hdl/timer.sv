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
  //always_comb out = out_val(rd, reg_sel, control_reg, interval_reg, timer_reg);
  assign out = rd ? sel_reg_val(reg_sel, control_reg, interval_reg, timer_reg) : 'hz;

  // writes
  always_ff @(posedge clk) begin
    if (wr) begin
      unique case (reg_sel)
        timer_pkg::ACK: timeout <= 'b0;
        timer_pkg::CTRL: control_reg <= timer_ctrl_t'(data[1:0]);
        timer_pkg::INTERVAL: interval_reg <= data;
        timer_pkg::TIMER: timer_reg <= data;
      endcase
    end
  end

  // starting
  always @(posedge control_reg._start) timer_reg <= interval_reg;

  // decrement count
  always_ff @(posedge clk) begin
    if (control_reg._start) begin
      timer_reg <= timer_reg - 1;

      if (timer_reg - 1 === 'b0) begin
        timeout <= 1;

        // if continue is set, start again, else set stop (start = 0)
        if (control_reg._continue) timer_reg <= interval_reg;
        else control_reg[0] <= 0;
      end
    end
  end

  // just a function to get the selected register val to assign to the output
  function static logic [31:0] sel_reg_val(
      input timer_reg_e reg_sel, input timer_ctrl_t control_reg,
      input logic [WORD_SIZE-1:0] interval_reg, input logic [WORD_SIZE-1:0] timer_reg);
    unique case (reg_sel)
      timer_pkg::ACK: sel_reg_val = 'b0;
      timer_pkg::CTRL: sel_reg_val = control_reg;
      timer_pkg::INTERVAL: sel_reg_val = interval_reg;
      timer_pkg::TIMER: sel_reg_val = timer_reg;
    endcase
  endfunction
endmodule
