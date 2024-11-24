	ld r1, '0';
loop:
	st r1, [0x4000]; // write r1
	add r1, 1;
	sub.t r1, '9';
	ld.leu pc, loop;
	ld pc, test;
end:
	ld pc, end;

test:
	ld r2, [thing - 1];
	ld pc, lr;

'\n';
thing: '9';

whatthe: 0x123+321;
