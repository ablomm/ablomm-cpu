/*
prints 0 to 9 to the tty
*/

import * from "lib/defines.asm";

	num = r0; // alias num to r0
	new_line = r1; // alias new_line to r1

	ld num, '0'; // load number with the ascii of 0
loop:
	ld tty, num; // print the ascii num to the terminal
	add num, 1; // get next ascii character
	sub.t num, '9'; // test if the number is ascii 9
	ld.ule pc, loop; // if previous test is less than or equal (unsigned), then loop

	ld new_line, '\n'; // load a newline character
	ld tty, new_line; // print the newline character

	// shutdown the cpu
	ld r0, power_shutdown_code;
	ld power, r0;
