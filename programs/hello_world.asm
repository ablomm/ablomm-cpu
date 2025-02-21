import * from "lib/defines.asm";
import print from "lib/print.asm";

	ld r0, string1;
	ld pc, print;
	ld r0, string2;
	ld pc, print;

	ld r0, SHUTDOWN;
	ld power, r0;

string1: "Hello world!👻\n\0";
string2: "Hello world, again!😵\n\0";
