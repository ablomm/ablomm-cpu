module mem_tb;
	reg[15:0] addr;
	reg[31:0] data_reg;
	wire[31:0] data;
	reg rd, wr;

	assign data = wr ? data_reg : 'hz;

	mem m0 (
		.rd (rd),
		.wr(wr),
		.data(data),
		.addr(addr)
	);

	initial begin
		#10
		addr = 14;
		data_reg = 321;
		rd = 0;
		wr = 1;

		#10
		addr = 15;
		data_reg = 213;
		rd = 0;
		wr = 1;

		#10
		addr = 14;
		rd = 1;
		wr = 0;
		#10 $display("data = %d", data);
		if (data != 321) $finish(1);
	end
endmodule
