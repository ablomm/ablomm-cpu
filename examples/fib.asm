/*
prints first 10 fibonacci numbers, starting at 0
yes, I know this is incredibly inefficient
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

	loop_max = 10; // alias loop_map to 10
	i = r2; // alias i to r2

	// initalize i
	ld i, 0;

// loop to print each fibonacci number
loop:
	// call fib(i)
	push i;
	ld pc.link, fib;

	// r0 contains the result of fib(i)
	// so print it: print(r0)
	push r0;
	ld pc.link, print_num;

	// print a new line: print('\n')
	ld r0, '\n';
	ld tty, r0;

	add i, 1; // add 1 to i
	sub.t i, loop_max; // test if i is at loop_max
	ld.ne pc, loop; // if it's not equal to loop_max, then loop

return:
	// shutdown the cpu
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

		n_in = *(fp + 1); // alias for the input on the input stack
		
		// register aliases
		result = r0;
		n = r2;

		// put the input into a register
		ld n, n_in;

		// if n == 0;
		sub.t n, 0;
		ld.eq result, 0; // result is 0
		ld.eq pc, return; // branch to return

		// if n == 1;
		sub.t n, 1;
		ld.eq result, 1; // result is 1
		ld.eq pc, return; // branch to return

		// call fib(n-1)
		sub n, 1;
		push n;
		ld pc.link, fib; // recursion!

		// move the result out of r0 because future function calls with overwrite r0
		fib_n_minus_1 = r3;
		ld fib_n_minus_1, r0;

		// call fib(n-2)
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
