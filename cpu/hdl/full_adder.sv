module full_adder #(
    parameter integer WIDTH = 32
) (
    input [WIDTH-1:0] a,
    input [WIDTH-1:0] b,
    input carry_in,
    input borrow_in, // yes, I know this is technically not for a "full_adder", but it makes things simplier
    output logic [WIDTH-1:0] out,
    output logic carry_out,
    output logic overflow
);
  always_comb begin
    // I think we actually want the borrow to be sign extended here (up to WIDTH bits)
    {carry_out, out} = (WIDTH+1)'(a) + (WIDTH+1)'(b) + (WIDTH+1)'(carry_in) - (WIDTH+1)'(borrow_in);
    overflow = out[WIDTH-1] ^ a[WIDTH-1] ^ b[WIDTH-1] ^ carry_out;
  end
endmodule
