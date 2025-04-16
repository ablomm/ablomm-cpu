/*
hello world program using only characters
*/

import * from "lib/defines.asm";

	ld r0, 'H'; // load r0 with the ascii value of H
	ld tty, r0; // print it

	// do it for each character
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

	// 👻 emoji utf8 value
	ghost_emoji = 0xf09f91bb;
	ld r0, (ghost_emoji >> 24) & 0xff; // get most significant byte and print it
	ld tty, r0;
	ld r0, (ghost_emoji >> 16) & 0xff; // get next byte and print it
	ld tty, r0;
	ld r0, (ghost_emoji >> 8) & 0xff; // get next byte and print it
	ld tty, r0;
	ld r0, ghost_emoji & 0xff; // get last byte and print it
	ld tty, r0;

	// print a newline
	ld r0, '\n';
	ld tty, r0;

	// shutdown cpu
	ld r0, power_shutdown_code;
	ld power, r0;
