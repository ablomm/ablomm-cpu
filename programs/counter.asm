import tty_addr from "lib/defines.asm";

	ld r0, '0';
	ld r1, '\n';
loop:
	st r0, [tty_addr];
	add r0, 1;
	sub.t r0, '9';
	ld.leu pc, loop;
	st r1, [tty_addr];
end:
	ld pc, end;
