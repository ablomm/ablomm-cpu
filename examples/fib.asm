/*
	prints first 10 fibinatchi numbers, starting at 0
	yes, I know this is incredibly inefficient
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

loop_max = 10;

counter = r2;
	ld counter, 0;

loop:
	ld r1, counter;
	ld pc.link, fib;

	// r0 contains the result of fib(n)
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
// input: r1 = n
// output: r0 = fib(n)
fib: {
		push lr;
		push r1;
		push r2;
	
	n = r1;
	result = r0;

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
		ld pc.link, fib; // recursion!

	fib_n_minus_1 = r2;
		ld fib_n_minus_1, r0;

		// calculate fib(n-2)
		sub n, 1;
		ld pc.link, fib; // more recursion!
		
	fib_n_minus_2 = r0;

		add result, fib_n_minus_1, fib_n_minus_2;

	return:
		pop r2;
		pop r1;
		pop pc;
}
