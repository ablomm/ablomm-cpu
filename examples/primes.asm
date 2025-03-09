/*
	prints the prime numbers up to 100
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

loop_max = 100;

count = r1;
	ld count, 2;

loop:
	ld pc.link, is_prime;

is_prime_result = r0;

	sub.t is_prime_result, 1;
	ld.eq r0, count;
	ld.eq pc.link, print_num;
	ld.eq r0, '\n';
	ld.eq tty, r0;

	add count, 1;
	sub.t count, loop_max;
	ld.ne pc, loop;

return:
	ld r0, power_shutdown_code;
	ld power, r0;


// input: r1 = num
// output: r0 = boolean, 1 if num is prime, else 0
// note: assumes 1 is prime, although it is not actually prime
is_prime: {
	import * from "lib/num.asm";

		push lr;
		push status;
		push r1;
		push r2;
		push r3;
		push r4;

	result = r0;
	num = r4;
		ld num, r1;

	count = r3;
		ld count, 2;

	loop:
		// while (count * count < num) i.e. (count < sqrt(num))
		ld r2, count;
		ld pc.link, mul;
	count_squared = r0;

		sub.t count_squared, num;
		// if greater or equal, then we tested all values, so it's prime
		ld.uge result, 1;
		ld.uge pc, return;

		// divide by the count
		ld r2, num;
		ld pc.link, div;
	remainder = r1;

		// check if remainder is 0 (not prime)
		sub.t remainder, 0;
		ld.eq result, 0;
		ld.eq pc, return;

		add count, 1;
		ld pc, loop;

	return:
		pop r4;
		pop r3;
		pop r2;
		pop r1;
		pop status;
		pop pc;
}
