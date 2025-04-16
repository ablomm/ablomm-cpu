import * from "defines.asm";

// does long division (see https://en.wikipedia.org/wiki/Division_algorithm#Integer_division_(unsigned)_with_remainder)
// inputs: numerator, divisor
// output: r0 = quotient, r1 = remainder
export div: {
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push status;
		push r2;
		push r3;
		push r4;

		numerator_in = *(fp + 2); // alias for numerator on the input stack
		divisor_in = *(fp + 1); // alias for divisor on the input stack

		// register aliases
		quotient = r0;
		remainder = r1;
		divisor = r2;
		i = r3;

		// take stack values and put it in registers
		ld quotient, numerator_in; // this is an optimization
		ld divisor, divisor_in;

		// initialize values
		ld i, 32; // 32 bits per word
		ld remainder, 0;

	loop:
		// shift through carry
		shl remainder, 1;
		shl.s quotient, 1; // left shift quotient and set NZCV flags
		or.cs remainder, 1; // if last shift resulted in a carry, then set least significant bit of remainder to 1

		sub.s r4, remainder, divisor; // subtract remainder by divisor, and set NZCV flags
		ld.uge remainder, r4; // if remainder >= divisor, load remainder with result of the previous subtraction
		or.uge quotient, 1; // if remainder >= divisor, set the least significant bit of quotient to 1

		sub.s i, 1; // subtract one from i and set NZCV flags
		ld.zc pc, loop; // if last subtraction resulted in a non-zero value, then loop

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
	
		operand1_in = *(fp + 2); // alias for first operand on the input stack
		operand2_in = *(fp + 1); // alias for the second operand on the input stack

		// register aliases
		result_low = r0;
		result_high = r1;
		operand1 = r2;
		operand2 = r3;
		operand2_high = r4; // we will shift operand2 into this

		// take from the input stack and place in registers
		ld operand1, operand1_in;
		ld operand2, operand2_in;

		// initialize values
		ld result_low, 0;
		ld result_high, 0;

		ld operand2_high, 0;
	
	loop:
		shr.s operand1, 1; // shift operand1 by 1 and set NZCV flags
		ld.cc pc, skip; // if the shift resulted in no carry, then skip the next few instructions

		add.s result_low, operand2; // add result_low and operand2 and set NZCV flags
		add.cs result_high, 1; // add to the result_high if the previous add resulted in a carry
		add result_high, operand2_high; // add result_high and operand2_high
	skip:

		// shifts operand2 and operand2_high as if they were one bit 64 bit register
		shl.s operand2, 1; // shift operand2 by 1 and set NZCV flags
		shl operand2_high, 1; // shift operand2_high by 1
		or.cs operand2_high, 1; // if the operand2 shift resulted in a carry, then set the least significant bit of operand2_high to 1
		
		sub.t operand1, 0; // check if operand1 is 0
		ld.ne pc, loop; // if it is not 0, then loop

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
