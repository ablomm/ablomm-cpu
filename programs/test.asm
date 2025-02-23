tty = *0x4000; // the tty device
char_to_print = r0;
  ld char_to_print, 'H';
  ld tty, char_to_print; // prints a "H" to the terminal
  ld char_to_print, '\n';
  ld tty, char_to_print;

tty_address = &tty; // get the value 0x4000
  ld r0, tty_address; // r0 = 0x4000

  ld fp, 123;
local_variable = *(fp + 3);

  ld local_variable, r0; // the address fp+3 (126) how contains the value 0x4000

  ld r0, 5 << 2 + tty_address * 4;
  ld *(r1 + 3 * 2), r0; // the address r1 + 3 * 2 now contains the result of the expression 5 << 2 + tty_address * 4
