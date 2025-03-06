

	// rotate left through carry
	ld.cs r1, 1;
	ld.cc r1, 0;
	shl.s r0, 1;
	and r0, r1;
