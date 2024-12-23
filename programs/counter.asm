import tty from "lib/defines.asm";

num = r0;
new_line = r1;

	ld r0, '0';
loop:
	ld tty, num;
	add num, 1;
	sub.t num, '9';
	ld.ule pc, loop;

	ld new_line, '\n';
	ld tty, new_line;
end:
	ld pc, end;
