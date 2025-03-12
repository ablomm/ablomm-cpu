import * from "defines.asm";

// inputs: string to print
export print: {
		// setup stack frame
		ld *sp.dec, fp;
		ld fp, sp;

		// saved registers
		ld *sp.dec, status;
		ld *sp.dec, r2;

	string_ptr = r0;
	string_word = r1;
	bytes_left = r2;

		ld string_ptr, *(fp + 1);

	print_word:
		ld string_word, *string_ptr;
		ld bytes_left, 4; // 4 bytes in a word

	/* 
		since memory is only word addressible
		we need to do some rotates to get each byte
		individually
	*/

	print_byte:
		rol string_word, 8;
		and.t string_word, 0xff;
		ld.zs pc, return; // i.e. lsb is null '\0'
		ld tty, string_word;
		sub.s bytes_left, 1;
		ld.ne pc, print_byte;

		// we have printed all the bytes in the word
		add string_ptr, 1;
		ld pc, print_word;

	return:
		ld r2, *sp.inc;
		ld status, *sp.inc;

		ld sp, fp;
		ld fp, *sp.inc;

		// remove arguments
		add sp, 1;

		ld pc, lr;
}

// inputs: num to print
export print_num: {
	import div from "num.asm";
		// setup stack frame
		ld *sp.dec, fp;
		ld fp, sp;

		// saved registers
		ld *sp.dec, lr;
		ld *sp.dec, status;
		ld *sp.dec, r2;

	num = *(fp + 1);
		ld r0, num;
		ld *sp.dec, r0;

		ld r0, 10;
		ld *sp.dec, r0;

		ld pc.link, div;
	quotent = r0; // will contain all but the last digit of num
	remainder = r2; // will contain the last digit of num
		ld remainder, r1;

		// recursively print the remaning digits first
		sub.t quotent, '\0';
		ld.ne *sp.dec, quotent;
		ld.ne pc.link, print_num; // when quotoent is 0, we are done

		add remainder, '0'; // get ascii of digit
		ld tty, remainder;

	return:
		ld r2, *sp.inc;
		ld status, *sp.inc;
		ld lr, *sp.inc;

		ld sp, fp;
		ld fp, *sp.inc;

		// remove arguments
		add sp, 1;

		ld pc, lr;
}
