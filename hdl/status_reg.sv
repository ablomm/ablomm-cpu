import reg_pkg::*;

module status_reg #(
    parameter status_t INITIAL_VAL = 0
) (
    input clk,
    input rst,
    output tri [5:0] a,
    output tri [5:0] b,
    input [5:0] in,
    input oe_a,
    input oe_b,
    input ld,
    input alu_status_t alu_status_in,
    input ld_alu_status,
    input imask_in,
    input ld_imask,
    input cpu_mode_e mode_in,
    input ld_mode,
    output status_t value = INITIAL_VAL
);

  assign a = oe_a ? value : 'hz;
  assign b = oe_b ? value : 'hz;

  always @(posedge rst) value <= INITIAL_VAL;

  always_ff @(posedge clk) begin
    if (ld) value <= in;
    if (ld_alu_status) value.alu_status <= alu_status_in;
    if (ld_imask) value.imask <= imask_in;
    if (ld_mode) value.imask <= mode_in;
  end
endmodule
