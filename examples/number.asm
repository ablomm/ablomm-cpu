import * from "lib/defines.asm";
import print_num from "lib/print.asm";

	ld r0, *number;
	shl r0, 1;
	ld pc.link, print_num;

	ld r0, '\n';
	ld tty, r0;

	ld r0, power_shutdown_code;
	ld power, r0;

number: 1236784913;
