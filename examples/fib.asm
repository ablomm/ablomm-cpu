/*
prints first 10 fibonacci numbers, starting at 0
yes, I know this is incredibly inefficient
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

	loop_max = 10;
	i = r2;
	ld i, 0;

loop:
	push i;
	ld pc.link, fib;

	// r0 contains the result of fib(n)
	push r0;
	ld pc.link, print_num;

	ld r0, '\n';
	ld tty, r0;

	add i, 1;
	sub.t i, loop_max;
	ld.ne pc, loop;

return:
	ld r0, power_shutdown_code;
	ld power, r0;


// calculates the n'th fibonacci number
// inputs: n
// outputs: r0 = fib(n)
fib: {
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push lr;
		push status;
		push r2;
		push r3;

		n_in = *(fp + 1);
		
		result = r0;
		n = r2;

		ld n, n_in;

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
		push n;
		ld pc.link, fib; // recursion!

		fib_n_minus_1 = r3;
		ld fib_n_minus_1, r0;

		// calculate fib(n-2)
		sub n, 1;
		push n;
		ld pc.link, fib; // more recursion!
		
		fib_n_minus_2 = r0;

		add result, fib_n_minus_1, fib_n_minus_2;

	return:
		pop r3;
		pop r2;
		pop status;
		pop lr;

		ld sp, fp;
		pop fp;

		// remove arguments
		add sp, 1;

		ld pc, lr;
}
