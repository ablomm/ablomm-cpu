import "../programs/print.asm";

	ld r0, string1;
	ld pc, print;
	ld r0, string2;
	ld pc, print;
end:
	ld pc, end;

string1: "Hello world!\n\0";
string2: "Hello world, again!\n\0";
