import * from "defines.asm";

// does long divison
// inputs: numerator, divisor
// output: r0 = quotent, r1 = remainder
export div: {
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push status;
		push r2;
		push r3;
		push r4;
		push r5;

	quotent = r0;
	remainder = r1;
	numerator = r2;
	divisor = r3;
	counter = r4;

		ld numerator, *(fp + 2);
		ld divisor, *(fp + 1);

		ld counter, 32;

		ld quotent, numerator;
		ld remainder, 0;

	shift_numerator:
		shl.s quotent, 1;
		shl remainder, 1;
		or.cs remainder, 1;

		sub.s r5, remainder, divisor;
		ld.cs remainder, r5;
		or.cs quotent, 1;

		sub.s counter, 1;
		ld.zc pc, shift_numerator;


	return:
		pop r5;
		pop r4;
		pop r3;
		pop r2;
		pop status;

		ld sp, fp;
		pop fp;

		// remove arguments
		add sp, 2;

		ld pc, lr;
}

// inputs: operand1, operand2
// output: r0 = result low, r1 = result high
export mul: {
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push status;
		push r2;
		push r3;
		push r4;

	result_low = r0;
	result_high = r1;
	operand1 = r2;
	operand2 = r3;
	operand2_high = r4;

		ld operand1, *(fp + 2);
		ld operand2, *(fp + 1);

		ld result_low, 0;
		ld result_high, 0;

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
		pop r3;
		pop r2;
		pop status;

		ld sp, fp;
		pop fp;
		
		// remove arguments
		add sp, 2;

		ld pc, lr;
}
