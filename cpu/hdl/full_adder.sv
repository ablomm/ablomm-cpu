module full_adder #(
    parameter integer WIDTH = 32
) (
    input [WIDTH-1:0] a,
    input [WIDTH-1:0] b,
    output logic [WIDTH-1:0] out,
    output logic carry_out,
    output logic overflow
);
  always_comb begin
    {carry_out, out} = (WIDTH + 1)'(a) + (WIDTH + 1)'(b);
    overflow = out[WIDTH-1] ^ a[WIDTH-1] ^ b[WIDTH-1] ^ carry_out;
  end
endmodule
