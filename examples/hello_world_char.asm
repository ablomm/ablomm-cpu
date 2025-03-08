/*
	hello world program using only characters
*/

import * from "lib/defines.asm";

	ld r0, 'H';
	ld tty, r0;

	ld r0, 'e';
	ld tty, r0;

	ld r0, 'l';
	ld tty, r0;
	ld tty, r0;

	ld r0, 'o';
	ld tty, r0;

	ld r0, ' ';
	ld tty, r0;

	ld r0, 'w';
	ld tty, r0;

	ld r0, 'o';
	ld tty, r0;

	ld r0, 'r';
	ld tty, r0;

	ld r0, 'l';
	ld tty, r0;

	ld r0, 'd';
	ld tty, r0;

	ld r0, '!';
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

	ld r0, power_shutdown_code;
	ld power, r0;
