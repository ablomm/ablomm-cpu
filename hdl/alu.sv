import alu_pkg::*;

module alu (
    input oe,
    input alu_op_e operation,
    input carry_in,
    input [31:0] a,
    input [31:0] b,
    output tri [31:0] out,
    output alu_status_t status
);

  logic [31:0] out_var;

  assign out = oe ? out_var : 'hz;

  always_comb begin
    unique case (operation)
      alu_pkg::PASSA: out_var = a;
      alu_pkg::PASSB: out_var = b;

      alu_pkg::AND: out_var = a & b;
      alu_pkg::OR:  out_var = a | b;
      alu_pkg::XOR: out_var = a ^ b;
      alu_pkg::NOT: out_var = ~a;

      alu_pkg::ADD: {status.carry, out_var} = a + b;
      alu_pkg::ADDC: {status.carry, out_var} = a + b + carry_in;
      alu_pkg::SUB: {status.carry, out_var} = a - b;
      alu_pkg::SUBB: {status.carry, out_var} = a - b - ~carry_in;
      alu_pkg::SHL: {status.carry, out_var} = a << b;
      alu_pkg::SHR: out_var = a >> b;
      alu_pkg::ASHR: out_var = a >>> b;
      default: out_var = 0;
    endcase
  end

  always @(out_var) begin
    // negative
    status.negative = out_var[31];

    // zero
    status.zero = out_var === 0;

    // overflow
    status.overflow = out_var[31] ^ a[31] ^ b[31] ^ status.carry;
  end
endmodule
