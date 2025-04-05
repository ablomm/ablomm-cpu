/*
prints 0 to 9 to the tty
*/

import * from "lib/defines.asm";

	num = r0;
	new_line = r1;

	ld num, '0';
loop:
	ld tty, num;
	add num, 1;
	sub.t num, '9';
	ld.ule pc, loop;

	ld new_line, '\n';
	ld tty, new_line;

	ld r0, power_shutdown_code;
	ld power, r0;
