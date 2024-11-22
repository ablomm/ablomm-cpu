module rom #(
    parameter integer WORD_SIZE = 32,
    parameter integer ADDR_WIDTH = 16,
    parameter integer DEPTH = 2 ** ADDR_WIDTH
) (
    input clk,
    input [ADDR_WIDTH-1:0] addr,
    output tri [WORD_SIZE-1:0] out,
    input rd,
    input en
);

  logic [WORD_SIZE-1:0] mem[DEPTH];

  assign out = (en && rd) ? mem[addr] : 'hz;
  initial $readmemh("simulation/programs/program.txt", mem);
endmodule
