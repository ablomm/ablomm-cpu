import tty from "lib/defines.asm";

num = r0;
new_line = r1;

	ld r0, '0';
loop:
	st num, tty;
	add num, 1;
	sub.t num, '9';
	ld.leu pc, loop;

	ld new_line, '\n';
	st new_line, tty;
end:
	ld pc, end;
