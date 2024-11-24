	ld r0, '0';
	ld r1, '\n';
loop:
	st r0, [0x4000];
	add r0, 1;
	sub.t r0, '9';
	ld.leu pc, loop;
	st r1, [0x4000];
end:
	ld pc, end;
