module counter_cpu_reg_tb;
  logic clk;
  wire [31:0] a;
  wire [31:0] b;
  logic [31:0] in;
  logic oe_a;
  logic oe_b;
  logic ld;
  logic rst;
  logic cnt;

  counter_cpu_reg counter0 (
      .clk(clk),
      .a (a),
      .b (b),
	  .in(in),
      .oe_a (oe_a),
      .oe_b (oe_b),
      .ld (ld),
      .rst(rst),
      .cnt(cnt)
  );

  initial begin
    reset();
    set_oe();
    count();
    $display("count = %d", a);
    if (a !== 1) $finish(1);
    count();
    $display("count = %d", a);
    if (a !== 2) $finish(1);
    set(32'hffffffff);
    $display("count = %d", a);
    if (a !== 32'hffffffff) $finish(1);
    count();
    $display("count = %d", a);
    if (a !== 0) $finish(1);
  end

  task static count();
    begin
      clk = 0;
      #1;
      cnt = 1;
      #1;
      clk = 1;
      #1;
    end
  endtask

  task static set(logic [31:0] in_in);
    begin
      clk = 0;
      #1;
      in  = in_in;
      ld  = 1;
      clk = 1;
      #1;
      ld = 0;
    end
  endtask

  task static reset();
    begin
      clk = 0;
      #1;
      rst = 1;
      clk = 1;
      #1;
      rst = 0;
    end
  endtask

  task static set_oe();
    begin
      oe_a = 1;
      #1;
    end
  endtask


endmodule
