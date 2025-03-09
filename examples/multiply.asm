/*
	multiplies two numbers and prints the result
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";
import mul from "lib/num.asm";

	ld r2, *operand1;
	ld r3, *operand2;
	ld pc.link, mul;

	// r0 contains the result of the multiplication
	ld pc.link, print_num;

	ld r0, '\n';
	ld tty, r0;

	ld r0, power_shutdown_code;
	ld power, r0;

operand1: 123123;
operand2: 23;
