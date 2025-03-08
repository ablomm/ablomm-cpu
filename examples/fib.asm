/*
	prints first 10 fibinatchi numbers, starting at 0
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

loop_max = 10;

counter = r2;
	ld counter, 0;

loop:
	ld r0, counter;
	ld pc.link, fib;
	ld r0, r1;
	ld pc.link, print_num;

	ld r0, '\n';
	ld tty, r0;

	add counter, 1;
	sub.t counter, loop_max;
	ld.ne pc, loop;

return:
	ld r0, power_shutdown_code;
	ld power, r0;


// calculates the n'th fibinatchi number
// input: r0 = n
// output: r1 = fib(n)
fib: {
	result = r1;

		push lr;
		push r0;
		push r2;
	
	n = r0;
		ld n, r0;

		// if n == 0;
		sub.t n, 0;
		ld.eq result, 0;
		ld.eq pc, return;

		// if n == 1;
		sub.t n, 1;
		ld.eq result, 1;
		ld.eq pc, return;

		// calculate fib(n-1)
		sub n, 1;
		ld pc.link, fib;

	fib_n_minus_1 = r2;
		ld fib_n_minus_1, r1;

		// calculate fib(n-2)
		sub n, 1;
		ld pc.link, fib;
		
	fib_n_minus_2 = r1;

		add result, fib_n_minus_1, fib_n_minus_2;

	return:
		pop r2;
		pop r0;
		pop pc;
}
