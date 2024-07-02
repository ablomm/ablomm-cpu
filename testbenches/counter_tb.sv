module counter_tb;
  logic clk;
  logic rst;
  logic cnt;
  logic ld;
  logic oe;
  logic [31:0] in;
  wire [31:0] out;

  counter counter0 (
      .clk(clk),
      .rst(rst),
      .cnt(cnt),
      .ld (ld),
      .oe (oe),
      .in (in),
      .out(out)
  );

  initial begin
    reset();
    set_oe();
    count();
    $display("count = %d", out);
    if (out !== 1) $finish(1);
    count();
    $display("count = %d", out);
    if (out !== 2) $finish(1);
    set(32'hffffffff);
    $display("count = %d", out);
    if (out !== 32'hffffffff) $finish(1);
    count();
    $display("count = %d", out);
    if (out !== 0) $finish(1);
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
      oe = 1;
      #1;
    end
  endtask


endmodule
