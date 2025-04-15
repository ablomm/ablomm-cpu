/*
prints the prime numbers up to 100
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

	loop_max = 100;

	i = r2;
	ld i, 2;

loop:
	push i;
	ld pc.link, is_prime;
	is_prime_result = r0;

	sub.t is_prime_result, 1;
	push.eq i;
	ld.eq pc.link, print_num;
	ld.eq r0, '\n';
	ld.eq tty, r0;

	add i, 1;
	sub.t i, loop_max;
	ld.ne pc, loop;

return:
	ld r0, power_shutdown_code;
	ld power, r0;


// inputs: num
// outputs: r0 = 1 if num is prime, else 0
// note: assumes 1 is prime, although it is not actually prime
is_prime: {
	import * from "lib/num.asm";
	
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push lr;
		push status;
		push r2;
		push r3;

		num_in = *(fp + 1);

		result = r0;
		num = r2;
		count = r3;

		ld num, num_in;

		ld count, 2;

	loop:
		// while (count * count <= num) i.e. (count <= sqrt(num))
		push count;
		push count;
		ld pc.link, mul;
		count_squared = r0;

		sub.t count_squared, num;
		// if greater or equal, then we tested all values, so it's prime
		ld.ugt result, 1;
		ld.ugt pc, return;

		// divide by the count
		push num;
		push count;
		ld pc.link, div;
		remainder = r1;

		// check if remainder is 0 (not prime)
		sub.t remainder, 0;
		ld.eq result, 0;
		ld.eq pc, return;

		add count, 1;
		ld pc, loop;

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
