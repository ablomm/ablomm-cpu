/*
prints the prime numbers up to 100
*/

import * from "lib/defines.asm";
import print_num from "lib/print.asm";

	loop_max = 100; // alias loop_max to 100

	i = r2; // alias i to r2
	ld i, 2; // initialize i

// checks if i is prime and print it
loop:
	// call is_prime(i)
	push i;
	ld pc.link, is_prime;
	is_prime_result = r0; // result is stored in r0

	sub.t is_prime_result, 1; // check if the result is 1 (is prime)

	// if it is, then print(i) 
	push.eq i;
	ld.eq pc.link, print_num;
	ld.eq r0, '\n';
	ld.eq tty, r0;

	add i, 1; // add one to 1
	sub.t i, loop_max; // test if i is at loop_max
	ld.ne pc, loop; // if it is not, then loop

return:
	// shutdown cpu
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

		num_in = *(fp + 1); // alias for input on input stack

		// register aliases
		result = r0;
		num = r2;
		count = r3;

		// put input into a register
		ld num, num_in;

		// initalize count
		ld count, 2;

	loop:
		// while (count * count <= num) i.e. (count <= sqrt(num))
		// get count * count
		push count;
		push count;
		ld pc.link, mul;
		count_squared = r0; // result is stored in r0

		sub.t count_squared, num; // test if count_squared is equal to num
		// if greater than (unsigned), then we tested all values, so it's prime
		ld.ugt result, 1; // set result to 1 (true)
		ld.ugt pc, return;

		// divide by the count (i.e. num / count)
		push num;
		push count;
		ld pc.link, div;
		remainder = r1; // remainder is in r1

		// check if remainder is 0 (not prime)
		sub.t remainder, 0;
		ld.eq result, 0; // set result to 0 (false)
		ld.eq pc, return;

		// loop
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
