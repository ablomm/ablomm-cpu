import * from "defines.asm";

// inputs: string to print
export print: {
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push status;
		push r2;

		string_ptr_in = *(fp + 1); // alias for the string on the input stack

		string_ptr = r0; // the pointer to the string, in a register (rather than on the stack)
		string_word = r1; // holds a word of string characters to print (4 characters per word)
		bytes_left = r2; // how many bytes in the word is left to print

		ld string_ptr, string_ptr_in; // get the input and put it in a register

	// print every character in the string_word
	print_word:
		ld string_word, *string_ptr; // get the word string_ptr is pointing at
		ld bytes_left, 4; // 4 bytes in a word

	/* 
	since memory is only word addressable
	we need to do some rotates to get each byte
	individually
	*/

	// print a single byte from the string_word
	print_byte:
		rol string_word, 8; // get the most significant byte on the lower 8 bits (to print it in the correct order)
		and.t string_word, 0xff; // test the byte to print
		ld.zs pc, return; // i.e. lsb is null '\0' then return (we are done)
		ld tty, string_word; // print the least significant byte
		sub.s bytes_left, 1; // subtract bytes_left and set the status flags
		ld.ne pc, print_byte; // if bytes_left didn't equal 1 before the subtraction, then print the next byte

		// we have printed all the bytes in the word
		add string_ptr, 1; // get the next word in the string
		ld pc, print_word; // branch to print the word

	return:
		pop r2;
		pop status;

		ld sp, fp;
		pop fp;

		// remove arguments
		add sp, 1;

		ld pc, lr;
}

// inputs: num to print
export print_num: {
	import div from "num.asm";
		// setup stack frame
		push fp;
		ld fp, sp;

		// saved registers
		push lr;
		push status;
		push r2;

		num_in = *(fp + 1); // alias for the number to print on the input stack

		ld r0, num_in; // put the input on the stack into a reigster
		push r0;

		ld r0, 10;
		push r0;

		ld pc.link, div; // divide number by 10
		quotient = r0; // will contain all but the last digit of num
		remainder = r2; // will contain the last digit of num

		ld remainder, r1; // we need to mvoe this into r2 because r1 will be changed by other functions

		// recursively print the remaining digits first
		sub.t quotient, 0; // test if the quotient is 0

		// if it's not then recursively print the remaining digits
		push.ne quotient;
		ld.ne pc.link, print_num;

		add remainder, '0'; // get ascii of digit
		ld tty, remainder; // print it

	return:
		pop r2;
		pop status;
		pop lr;

		ld sp, fp;
		pop fp;

		// remove arguments
		add sp, 1;

		ld pc, lr;
}
