module alu (
    input oe,
    input [3:0] operation,
    input carry_in,
    input [31:0] a,  // data bus
    input [31:0] b,  // addr bus
    output tri [31:0] out,  // result bus
    output logic [3:0] status  // NZCV
);

  `include "include/alu_op.vh"

  logic [31:0] out_reg = 'hz;

  assign out = oe ? out_reg : 'hz;

  always_comb begin
    case (operation)
      `ALU_PASSA: out_reg = a;
      `ALU_PASSB: out_reg = b;

      `ALU_AND: out_reg = a & b;
      `ALU_OR: out_reg = a | b;
      `ALU_XOR: out_reg = a ^ b;
      `ALU_NOT: out_reg = ~a;

      `ALU_ADD: {status[1], out_reg} = a + b;
      `ALU_ADDC: {status[1], out_reg} = a + b + carry_in;
      `ALU_SUB: {status[1], out_reg} = a - b;
      `ALU_SUBB: {status[1], out_reg} = a - b - ~carry_in;
      `ALU_SHL: {status[1], out_reg} = a << 1;
      `ALU_SHR: {status[1], out_reg} = a >> 1;
      `ALU_ASHR: {status[1], out_reg} = a >>> 1;
      default: out_reg = 0;
    endcase
  end

  always @(out_reg) begin
    // negative
    status[3] = out_reg[31] == 1;

    // zero
    status[2] = out_reg == 0;

    // overflow
    status[0] = out_reg[31] ^ a[31] ^ b[31] ^ status[1];

  end
endmodule
