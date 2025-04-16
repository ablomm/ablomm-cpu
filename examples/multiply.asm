/*
multiplies two numbers and prints the result
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";
import mul from "lib/num.asm";

	// load operand1 into r0 and push it
	ld r0, *operand1;
	push r0;

	// load operand2 into r0 and push it
	ld r0, *operand2;
	push r0;

	// call mul (i.e. operand1 * operand2)
	ld pc.link, mul;

	mul_result = r0; // alias mul_result to r0

	// call print(mul_result)
	push mul_result;
	ld pc.link, print_num;

	// call print('\n')
	ld r0, '\n';
	ld tty, r0;

	// shutdown the cpu
	ld r0, power_shutdown_code;
	ld power, r0;

operand1: 12;
operand2: 23;
