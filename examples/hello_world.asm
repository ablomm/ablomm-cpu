/*
prints a few strings
*/

import * from "lib/defines.asm";
import print from "lib/print.asm";

	// call print(string1)
	ld r0, string1; // load r0 with the pointer of string1
	push r0; // put it on the stack, as print will expect the input on the stack
	ld pc.link, print; // branch to print but also set the link register (lr) to allow print to return to the correcct address

	// do it again, but print string2! (i.e. print(string2)
	ld r0, string2;
	push r0;
	ld pc.link, print;

	// shutdown cpu
	ld r0, power_shutdown_code;
	ld power, r0;

string1: "Hello world!👻\n\0";
string2: "Hello world, again!😵\n\0";
