import alu_pkg::*;

module alu #(
    parameter integer WIDTH = 32
) (
    input oe,
    input alu_op_e operation,
    input carry_in,
    input [WIDTH-1:0] a,
    input [WIDTH-1:0] b,
    output tri [WIDTH-1:0] out,
    output alu_status_t status
);
  logic [WIDTH-1:0] out_var;

  assign out = oe ? out_var : 'hz;

  logic [WIDTH-1:0] adder_a, adder_b, adder_out;
  logic adder_carry_in, adder_borrow_in;
  wire adder_carry_out, adder_overflow;
  full_adder adder (
      .a(adder_a),
      .b(adder_b),
      .carry_in(adder_carry_in),
      .borrow_in(adder_borrow_in),
      .out(adder_out),
      .carry_out(adder_carry_out),
      .overflow(adder_overflow)
  );

  always_comb begin
    status = 0;
    adder_carry_in = 0;
    adder_borrow_in = 0;
    adder_a = a;
    adder_b = b;

    unique case (operation)
      // unary operations are always on b because it makes the assembler simpler
      // (i.e. "<OP> r1, r2" becomes "<OP> r1, r1, r2" which works for both
      // unary and binary operations
      alu_pkg::PASS: out_var = b;

      alu_pkg::AND: out_var = a & b;
      alu_pkg::OR:  out_var = a | b;
      alu_pkg::XOR: out_var = a ^ b;
      alu_pkg::NOT: out_var = ~b;

      // a + b
      alu_pkg::ADD: begin
        out_var = adder_out;
        status.carry = adder_carry_out;
        status.overflow = adder_overflow;
      end

      // a + b + carry_in
      alu_pkg::ADDC: begin
        adder_carry_in = carry_in;
        out_var = adder_out;
        status.carry = adder_carry_out;
        status.overflow = adder_overflow;
      end

      //  a - b
      alu_pkg::SUB: begin
        adder_b = -b;
        out_var = adder_out;
        status.carry = adder_carry_out;
        status.overflow = adder_overflow;
      end

      // a - b - ~carry_in
      // borrow is simply the not of carry
      alu_pkg::SUBB: begin
        adder_b = -b;
        adder_borrow_in = ~carry_in;
        out_var = adder_out;
        status.carry = adder_carry_out;
        status.overflow = adder_overflow;
      end

      alu_pkg::NEG:  out_var = -b;
      alu_pkg::SHL:  {status.carry, out_var} = (WIDTH + 1)'(a) << (WIDTH + 1)'(b);
      alu_pkg::SHR:  out_var = a >> b;
      alu_pkg::ASHR: out_var = $signed(a) >>> b;

      default: out_var = 0;
    endcase

    status.negative = out_var[31];
    status.zero = out_var === 0;
  end
endmodule
