import * from "defines.asm";

// does long divison
// input: r0 = numerator, r1 = divisor
// output: r2 = quotent, r3 = remainder
export div: {
		push r4;
		push r5;

	numerator = r0;
	divisor = r1;

	result_low = r2;
	result_high = r3;

	counter = r4;
		ld counter, 32;

		ld result_low, numerator;
		ld result_high, 0;

	shift_numerator:
		shl.s result_low, 1;
		shl result_high, 1;
		or.cs result_high, 1;

		sub.s r5, result_high, divisor;
		ld.cs result_high, r5;
		or.cs result_low, 1;

		sub.s counter, 1;
		ld.ne pc, shift_numerator;

	return:
		pop r5;
		pop r4;
		ld pc, lr;
}

// input r0, r1 = operands
// output: r2 = result low, r3 = result high
export mul: {
		push r0;
		push r1;
		push r4;

	operand1 = r0;
	operand2 = r1;
	result_low = r2;
	result_high = r3;

		ld result_low, 0;
		ld result_high, 0;

	operand2_high = r4;
		ld operand2_high, 0;
	
	loop:
		shr.s operand1, 1;
		add.cs result_low, operand2;
		add.cs result_high, operand2_high;

		shl.s operand2, 1;
		shl operand2_high, 1;
		or.cs operand2_high, 1;
		
		sub.t operand1, 0;
		ld.ne pc, loop;

	return:
		pop r4;
		pop r1;
		pop r0;
		ld pc, lr;
}
