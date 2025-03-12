import * from "lib/defines.asm";
ld *r3, r1;

ld r1, 2;
ld r3, 0x9000;

ld *r3, r1;
ld r0, *r3;

add r0, '0';

ld tty, r0;
ld r0, '\n';
ld tty, r0;

ld r0, power_shutdown_code;
ld power, r0;
