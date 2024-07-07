import ablomm_cpu::*;

module alu (
    input oe,
    input alu_op_e operation,
    input carry_in,
    input [31:0] a,  // data bus
    input [31:0] b,  // addr bus
    output tri [31:0] out,  // result bus
    output status_t status  // NZCV
);

  logic [31:0] out_reg = 'hz;

  assign out = oe ? out_reg : 'hz;

  always_comb begin
    case (operation)
      PASSA: out_reg = a;
      PASSB: out_reg = b;

      AND: out_reg = a & b;
      OR:  out_reg = a | b;
      XOR: out_reg = a ^ b;
      NOT: out_reg = ~a;

      ADD: {status.carry, out_reg} = a + b;
      ADDC: {status.carry, out_reg} = a + b + carry_in;
      SUB: {status.carry, out_reg} = a - b;
      SUBB: {status.carry, out_reg} = a - b - ~carry_in;
      SHL: {status.carry, out_reg} = a << 1;
      SHR: out_reg = a >> 1;
      ASHR: out_reg = a >>> 1;
      default: out_reg = 0;
    endcase
  end

  always @(out_reg) begin
    // negative
    status.negative = out_reg[31] == 1;

    // zero
    status.zero = out_reg == 0;

    // overflow
    status.overflow = out_reg[31] ^ a[31] ^ b[31] ^ status[1];

  end
endmodule
