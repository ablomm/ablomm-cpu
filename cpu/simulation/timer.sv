module timer #(
    parameter integer WORD_SIZE = 32
) (
    input clk,
    input [WORD_SIZE-1:0] data,
    input [1:0] reg_sel,
    input rd,
    input wr,
    output logic timeout
);
  logic [1:0] control_register;  // bit 0, start bit, bit 1 continue bit
  logic [WORD_SIZE-1:0] interval_register;

  logic [WORD_SIZE-1:0] timer;

  always_ff @(posedge clk) begin
    if (wr) begin
      unique case (reg_sel)
        'b00: timeout <= 0;
        'b01: control_register <= data[1:0];
        'b10: begin
          interval_register <= data;
          timer <= data;
        end
        default: ;
      endcase
    end
  end

  always_ff @(posedge clk) begin
    // if start bit is set
    if (control_register[0] === 'b1) begin
      timer <= timer - 1;

      if (timer - 1 === 'b0) begin
        timeout <= 1;

        // if continue is set, start again, else set stop (start = 0)
        if (control_register[1] === 'b1) timer <= interval_register;
        else control_register[0] <= 0;
      end
    end
  end
endmodule
