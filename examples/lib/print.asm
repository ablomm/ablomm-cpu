import * from "defines.asm";

// params: r0 = string to be printed
export print: {
		push r1;
		push r2;

	string_ptr = r0;
	string_word = r1;
	bytes_left = r2;

	print_word:
		ld string_word, *string_ptr;
		ld bytes_left, 4; // 4 bytes in a word

	/* 
		since memory is only word addressible
		we need to do some shifts to get each byte
		individually
	*/

	print_byte:
		and.t string_word, 0xff;
		ld.eq pc, return; // i.e. lsb is null '\0'
		ld tty, string_word;
		shr string_word, 8;
		sub.s bytes_left, 1;
		ld.ne pc, print_byte;
		// we have printed all the bytes in the word
		add string_ptr, 1;
		ld pc, print_word;
	return:
		pop r2;
		pop r1;
		ld pc, lr;
}

// r0 = num
export print_num: {
	import div from "num.asm";

		push lr;
		push r0;
		push r1;
		push r2;
		push r3;
	num = r0;
		ld r1, 10;
		ld pc.link, div;
	quotent = r2;
	remainder = r3;

		ld.s num, quotent;
		ld.zc pc.link, print_num;

		add remainder, remainder, '0';
		ld tty, remainder;

	return:
		pop r3;
		pop r2;
		pop r1;
		pop r0;
		pop pc;
}
