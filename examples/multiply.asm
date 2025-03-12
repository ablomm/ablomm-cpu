/*
	multiplies two numbers and prints the result
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";
import mul from "lib/num.asm";

	ld r0, *operand1;
	ld *sp.dec, r0;
	ld r0, *operand2;
	ld *sp.dec, r0;
	ld pc.link, mul;

mul_result = r0;
	ld *sp.dec, mul_result;
	ld pc.link, print_num;

	ld r0, '\n';
	ld tty, r0;

	ld r0, power_shutdown_code;
	ld power, r0;

operand1: 12;
operand2: 23;
