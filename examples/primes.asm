/*
	prints the prime numbers up to 100
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

loop_max = 100;

count = r2;
	ld count, 2;

loop:
	ld *sp.dec, count;
	ld pc.link, is_prime;

is_prime_result = r0;

	sub.t is_prime_result, 1;
	ld.eq *sp.dec, count;
	ld.eq pc.link, print_num;
	ld.eq r0, '\n';
	ld.eq tty, r0;

	add count, 1;
	sub.t count, loop_max;
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
		ld *sp.dec, fp;
		ld fp, sp;

		// saved registers
		ld *sp.dec, lr;
		ld *sp.dec, status;
		ld *sp.dec, r2;
		ld *sp.dec, r3;

	result = r0;
	num = r2;
	count = r3;

		ld num, *(fp + 1);

		ld count, 2;

	loop:
		// while (count * count < num) i.e. (count < sqrt(num))
		ld *sp.dec, count;
		ld *sp.dec, count;
		ld pc.link, mul;
	count_squared = r0;

		sub.t count_squared, num;
		// if greater or equal, then we tested all values, so it's prime
		ld.uge result, 1;
		ld.uge pc, return;

		// divide by the count
		ld *sp.dec, num;
		ld *sp.dec, count;
		ld pc.link, div;
	remainder = r1;

		// check if remainder is 0 (not prime)
		sub.t remainder, 0;
		ld.eq result, 0;
		ld.eq pc, return;

		add count, 1;
		ld pc, loop;

	return:
		ld r3, *sp.inc;
		ld r2, *sp.inc;
		ld status, *sp.inc;
		ld lr, *sp.inc;

		ld sp, fp;
		ld fp, *sp.inc;
		
		// remove arguments
		add sp, 1;

		ld pc, lr;
}
