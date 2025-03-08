/*
	multiplies two numbers and prints the result
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";
import mul from "lib/num.asm";

	ld r0, *operand1;
	ld r1, *operand2;
	ld pc.link, mul;

result = r2;

	ld r0, result;
	ld pc.link, print_num;

	ld r0, '\n';
	ld tty, r0;

	ld r0, power_shutdown_code;
	ld power, r0;

operand1: 123123;
operand2: 23;
