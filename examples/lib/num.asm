import * from "defines.asm";

// does long divison (see https://en.wikipedia.org/wiki/Division_algorithm#Integer_division_(unsigned)_with_remainder)
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

		numerator_in = *(fp + 2);
		divisor_in = *(fp + 1);

		quotent = r0;
		remainder = r1;
		divisor = r2;
		i = r3;

		ld quotent, numerator_in;
		ld divisor, divisor_in;

		ld i, 32;

		ld remainder, 0;

	loop:
		shl remainder, 1;
		shl.s quotent, 1;
		or.cs remainder, 1;

		sub.s r4, remainder, divisor;
		ld.uge remainder, r4;
		or.uge quotent, 1;

		sub.s i, 1;
		ld.zc pc, loop;

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

// does long multiplication (see https://en.wikipedia.org/wiki/Multiplication_algorithm#Long_multiplication)
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
	
		operand1_in = *(fp + 2);
		operand2_in = *(fp + 1);

		result_low = r0;
		result_high = r1;
		operand1 = r2;
		operand2 = r3;
		operand2_high = r4;

		ld operand1, operand1_in;
		ld operand2, operand2_in;

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
