/* 
	since memory is only word addressible
	we need to do some shifts to get each byte
	individually
*/

ld r0, string1;
ld pc, print;
ld r0, string2;
ld pc, print;
end:
	ld pc, end;

// params: r0 = string
print:
	push lr;
	push r1;
	push r2;
print_word:
	ld r1, [r0];
	ld r2, 4; // num of bytes in a word
print_byte:
	and.t r1, 0xff;
	ld.eq pc, return;
	st r1, [0x4000];
	shr r1, 8;
	sub.s r2, 1;
	ld.ne pc, print_byte;
	// we have printed all the bytes in the word
	add r0, 1;
	ld pc, print_word;
return:
	pop r2;
	pop r1;
	pop pc;

string1: "Hello world!\n\0";
string2: "Hello world, again!\n\0";

