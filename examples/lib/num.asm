import * from "defines.asm";

// input: r0 = numerator, r1 = divisor
// output: r2 = quotent, r3 = remainder
// does long divison
export div: {
		push r4;
		push r5;

		numerator = r0;
		divisor = r1;

		result_low = r2;
		result_high = r3;

		counter = r4;
		ld counter, 0;

		ld result_low, numerator;
		ld result_high, 0;

	shift_numerator:
		shl.s result_low, 1;
		shl result_high, 1;
		or.cs result_high, 1;

		sub.s r5, result_high, divisor;
		ld.cs result_high, r5;
		or.cs result_low, 1;

		add counter, 1;
		sub.t counter, 32;
		ld.ne pc, shift_numerator;

	return:
		pop r5;
		pop r4;
		ld pc, lr;
}

export mul: {


}
