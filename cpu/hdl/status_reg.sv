module status_reg
  import alu_pkg::*;
  import reg_pkg::*;
#(
    parameter status_t INITIAL_VAL = 0
) (
    input clk,
    input rst,
    output tri status_t a,
    output tri status_t b,
    input status_t in,
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
  // for some reason verilator complains when it's 'hz
  assign a = oe_a ? value : 6'hz;
  assign b = oe_b ? value : 6'hz;

  always_ff @(posedge clk or posedge rst)
    if (rst) value <= INITIAL_VAL;
    else begin
      if (ld && value.mode === reg_pkg::SUPERVISOR) value <= in;
      // only allow loading alu status if in user mode
      if (ld && value.mode === reg_pkg::USER) value.alu_status <= alu_status_in;
      if (ld_alu_status) value.alu_status <= alu_status_in;
      if (ld_imask) value.imask <= imask_in;
      if (ld_mode) value.mode <= mode_in;
    end
endmodule
