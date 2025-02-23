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
