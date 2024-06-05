module alu_tb;

  `include "include/alu_op.vh"

  reg oe;
  reg [31:0] a, b;
  wire [2:0] status;
  reg  [ 3:0] operation;
  wire [31:0] out;
  alu a1 (
      .oe(oe),
      .a(a),
      .b(b),
      .out(out),
      .status(status),
      .operation(operation),
      .carry_in(1'b0)
  );

  initial begin
    oe = 1;
    $display("testing addition:");
    operation = `ALU_ADD;

    a = 1;
    b = 1;
    #1 if (out !== 2) $finish(1);

    a = 2;
    b = 1;
    #1 if (out !== 3) $finish(1);

    a = 32'hffffffff;
    b = 2;
    #1 if (out !== 1 || status[0] !== 1) $finish(1);

    $display("\ntesting subtraction:");
    operation = `ALU_SUB;

    a = 1;
    b = 1;
    #1 if (out !== 0 || status[1] !== 1) $finish(2);

    a = 2;
    b = 1;
    #1 if (out !== 1) $finish(1);

    a = 2;
    b = 3;
    #1 if (out !== -1 || status[2] !== 1) $finish(1);

    $finish;
  end

  initial $monitor("a = %d, b = %d, out = %d, op = %h, status = %b", a, b, out, operation, status);

endmodule
