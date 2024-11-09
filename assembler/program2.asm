	ld r0 0x68; // 'h'
	st r0, [0x4000];
	ld r0, 0x65; // 'e'
	st r0, [0x4000];
	ld r0, 0x6c; // 'l'
	st r0, [0x4000];
	st r0, [0x4000];
	ld r0, 0x6f; // 'o'
	st r0, [0x4000];
	ld r0, 0x20; // ' '
	st r0, [0x4000];
	ld r0, 0x77; // 'w'
	st r0, [0x4000];
	ld r0, 0x6f; // 'o'
	st r0, [0x4000];
	ld r0, 0x72; // 'r'
	st r0, [0x4000];
	ld r0, 0x6c; // 'l'
	st r0, [0x4000];
	ld r0, 0x64; // 'd'
	st r0, [0x4000];
	ld r0, 0x21; // '!'
	st r0, [0x4000];
	ld r0, 0xa; // '\n'
	st r0, [0x4000];
end:
	ld pc, end;
