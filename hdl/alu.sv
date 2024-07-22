import alu_pkg::*;
import reg_pkg::*;

module alu (
    input oe,
    input alu_op_e operation,
    input carry_in,
    input [31:0] a,  // data bus
    input [31:0] b,  // addr bus
    output tri [31:0] out,  // result bus
    output alu_status_t status  // NZCV
);

  logic [31:0] out_reg = 'hz;

  assign out = oe ? out_reg : 'hz;

  always_comb begin
    case (operation)
      alu_pkg::PASSA: out_reg = a;
      alu_pkg::PASSB: out_reg = b;

      alu_pkg::AND: out_reg = a & b;
      alu_pkg::OR:  out_reg = a | b;
      alu_pkg::XOR: out_reg = a ^ b;
      alu_pkg::NOT: out_reg = ~a;

      alu_pkg::ADD: {status.carry, out_reg} = a + b;
      alu_pkg::ADDC: {status.carry, out_reg} = a + b + carry_in;
      alu_pkg::SUB: {status.carry, out_reg} = a - b;
      alu_pkg::SUBB: {status.carry, out_reg} = a - b - ~carry_in;
      alu_pkg::SHL: {status.carry, out_reg} = a << 1;
      alu_pkg::SHR: out_reg = a >> 1;
      alu_pkg::ASHR: out_reg = a >>> 1;
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
