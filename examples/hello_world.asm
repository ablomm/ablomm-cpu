/*
	prints a few strings
*/

import * from "lib/defines.asm";
import print from "lib/print.asm";

	ld r0, string1;
	ld *sp.dec, r0;
	ld pc.link, print;

	ld r0, string2;
	ld *sp.dec, r0;
	ld pc.link, print;

	ld r0, power_shutdown_code;
	ld power, r0;

string1: "Hello world!👻\n\0";
string2: "Hello world, again!😵\n\0";
