import * from "lib/defines.asm";

	ld r0, 'h';
	ld tty, r0;

// 👻 emoji utf8
ghost_emoji = 0xf09f91bb;
	ld r0, (ghost_emoji >> 24) & 0xff;
	ld tty, r0;
	ld r0, (ghost_emoji >> 16) & 0xff;
	ld tty, r0;
	ld r0, (ghost_emoji >> 8) & 0xff;
	ld tty, r0;
	ld r0, ghost_emoji & 0xff;
	ld tty, r0;

	ld r0, '\n';
	ld tty, r0;

	ld r0, SHUTDOWN;
	ld power, r0;
