	ld r1, '0';
	ld r2, '\n';
loop:
	st r1, [0x4000]; // write r1
	add r1, 1;
	sub.t r1, 58;
	ld.ne pc, loop;
	st r2, [0x4000]; // new line
end:
	ld pc, end;
