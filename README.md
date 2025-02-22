# Ablomm CPU and Assembler
This project contains a fully functioning 32 bit CPU written in SystemVerilog and an assembler for said CPU written in Rust.

The CPU can be simulated with Verilator or Icarus Verilog.

I have not synthesized it or ran it on an FPGA (because I don't have one right now), but it should all be synthesizable.

## Contents
- [Building and Running](#building-and-running)
- [Documentation](#documentation)
- [Assembler](#assembler)
	- [Key Features](#key-features)
 	- [Examples](#examples)
 
## Building and Running
To build and run this project see the [setup](docs/setup.md) document.

## Documentation
Please refer to the [docs directory](docs/) for the documentation on the assembly syntax and the CPU's ISA.

# Assembler
The assembler has quite a bit of featues inspired from high level languages such as C. By far the most advanced part of this project is the assembler, as I am a software guy more than a hardware one.
## Key Features

### File imports:

The assembler contains a fully fledged import and export system quite similar to JavaScript. Import aliasing, blob imports, and block scoped imports are all supported.

`lib/print.asm`:
``` asm
export print: {
  ...
}
```

`hello_world.asm`:
``` asm
import print from "lib/print.asm";

ld r0, string;
ld pc, print;

string: "hello world!\n\0";
```

---

### Blocks and lexical scopes

The assembler contains support for blocks and lexical scopes to avoid namespace collisions and logically group blocks of code.
``` asm
label: {
  identifier = 123;

  {
    identifier = identifier * 2; // shadows the parent identifier
    ld r0, identifier; // r0 = 246
  }

  ld r1, identifier; // r1 = 123
  add r1, r2; // r1 = 123 + 246
}

ld r2, identifier; // error: cannot find identifier!
ld r2, label; // r2 = address of label

```
---

### Beautiful error messages
``` asm
label: {
  identifier = 123;

  {
    identifier = identifier * 2; // shadows the parent identifier
    ld r0, identifier; // r0 = 246
  }

  ld r1, identifier; // r1 = 123
  add r1, r2; // r1 = 123 + 246
}

ld r2, identifier; // error: cannot find identifier!
ld r2, label; // r2 = address of label

```
![image](https://github.com/user-attachments/assets/bed91bf9-f8e8-414b-8f7d-6e7e06c0c66c)

``` asm
import * from "lib/print.asm";
print = 123;
```
![image](https://github.com/user-attachments/assets/7b8ce2c5-7be1-403a-9f54-5c1601878204)

---

### Compile time expressions

The assembler has support for compile time variables and expressions similar to c++ constexpr
``` asm
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
```

## Examples:
### Define a few variables
``` asm
export tty = *0x4000;
export power = *0x4001;
export SHUTDOWN = 0;
export RESTART = 1;
```

### Count from 0 to 9 and print it to the terminal:
``` asm
import * from "lib/defines.asm";

num = r0;
new_line = r1;

	ld r0, '0';
loop:
	ld tty, num;
	add num, 1;
	sub.t num, '9';
	ld.ule pc, loop;

	ld new_line, '\n';
	ld tty, new_line;

	ld r0, SHUTDOWN;
	ld power, r0;
```
![image](https://github.com/user-attachments/assets/a562133a-cbc3-48e3-945d-33867e017e60)


### Print a null terminated string to the terminal:
``` asm
import * from "defines.asm";

// params: r0 = string to be printed
export print: {
		push lr;
		push r1;
		push r2;

	string_ptr = r0;
	string_word = r1;
	bytes_left = r2;

	print_word:
		ld string_word, *string_ptr;
		ld bytes_left, 4; // 4 bytes in a word

	/* 
		since memory is only word addressible
		we need to do some shifts to get each byte
		individually
	*/

	print_byte:
		and.t string_word, 0xff;
		ld.eq pc, return; // i.e. lsb is null '\0'
		ld tty, string_word;
		shr string_word, 8;
		sub.s bytes_left, 1;
		ld.ne pc, print_byte;
		// we have printed all the bytes in the word
		add string_ptr, 1;
		ld pc, print_word;
	return:
		pop r2;
		pop r1;
		pop pc;
}
```

### Print hello world using the print function defined above:
``` asm
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
```
![image](https://github.com/user-attachments/assets/d3693ec4-e594-45c5-b75d-19c36e0dd057)
