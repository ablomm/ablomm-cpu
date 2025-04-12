# Ablomm CPU and Assembler

![ablomm_ghost](https://github.com/user-attachments/assets/490bea8d-e06b-4051-b459-b5ccc5217a4f)

This project contains a fully functioning 32-bit CPU written in SystemVerilog and an assembler for said CPU written in Rust.

The CPU can be simulated with Verilator or Icarus Verilog.

I have not synthesized it or ran it on an FPGA (because I don't have one right now), but it should all be synthesizable.

## Contents

- [Examples](#examples)
- [Building and Running](#building-and-running)
- [Documentation](#documentation)
- [Assembler](#assembler)
	- [Key Features](#key-features)
		- [File Imports](#file-imports)
		- [Compile Time Expressions](#compile-time-expressions)
		- [Blocks and Lexical Scopes](#blocks-and-lexical-scopes)
		- [Beautiful Error Messages](#beautiful-error-messages)

## Examples:

### Define a few constants:

```asm
export tty = *0x4006;
export power = *0x4005;
export power_shutdown_code = 0;
export power_restart_code = 1;
```

### Count from 0 to 9 and print it to the terminal:

```asm
import * from "lib/defines.asm";

    num = r0;
    new_line = r1;

    ld num, '0';
loop:
    ld tty, num;
    add num, 1;
    sub.t num, '9';
    ld.ule pc, loop;

    ld new_line, '\n';
    ld tty, new_line;

    ld r0, power_shutdown_code;
    ld power, r0;
```

![image](https://github.com/user-attachments/assets/d6a8093b-0f3b-4abb-8116-9a1a80520f6d)

### Print a null terminated string to the terminal:

```asm
import * from "defines.asm";

// inputs: string to print
export print: {
        // setup stack frame
        push fp;
        ld fp, sp;

        // saved registers
        push status;
        push r2;

        string_ptr_in = *(fp + 1);

        string_ptr = r0;
        string_word = r1;
        bytes_left = r2;

        ld string_ptr, string_ptr_in;

    print_word:
        ld string_word, *string_ptr;
        ld bytes_left, 4; // 4 bytes in a word

    /* 
    since memory is only word addressable
    we need to do some rotates to get each byte
    individually
    */

    print_byte:
        rol string_word, 8;
        and.t string_word, 0xff;
        ld.zs pc, return; // i.e. lsb is null '\0'
        ld tty, string_word;
        sub.s bytes_left, 1;
        ld.ne pc, print_byte;

        // we have printed all the bytes in the word
        add string_ptr, 1;
        ld pc, print_word;

    return:
        pop r2;
        pop status;

        ld sp, fp;
        pop fp;

        // remove arguments
        add sp, 1;

        ld pc, lr;
}
```

### Print hello world using the print function defined above:

```asm
import * from "lib/defines.asm";
import print from "lib/print.asm";

    ld r0, string1;
    push r0;
    ld pc.link, print;

    ld r0, string2;
    push r0;
    ld pc.link, print;

    ld r0, power_shutdown_code;
    ld power, r0;

string1: "Hello world!👻\n\0";
string2: "Hello world, again!😵\n\0";
```

![image](https://github.com/user-attachments/assets/30173b03-6720-46de-b36c-b3b418158c31)

For more examples, check out the [Examples directory](examples)!
 
## Building and Running

To build and run this project see the [setup](docs/setup.md) document.

## Documentation

Please refer to the [docs directory](docs/) for the documentation.

# Assembler

The assembler has quite a bit of features inspired from high level languages such as C. By far the most advanced part of this project is the assembler, as I am a software guy more than a hardware one.

## Key Features

### File Imports

The assembler contains a fully fledged import and export system quite similar to JavaScript. Import aliasing, blob imports, and block scoped imports are all supported.

`lib/print.asm`:

```asm
export print: {
  ...
}
```

`hello_world.asm`:

```asm
import print from "lib/print.asm";

    ld r0, string;
    ld pc.link, print;

string: "hello world!\n\0";
```

File imports are documented further in the [Imports and Exports document](docs/assembler/imports-and-exports.md).

---

### Compile Time Expressions

The assembler has support for compile time assignments and expressions similar to c++'s constexpr:

```asm
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

Compile time expressions are documented further in the [Expressions document](docs/assembler/expressions.md).

Compile time assignments are documented further in the [Assignments document](docs/assembler/assignments.md).

---

### Blocks and Lexical Scopes

The assembler contains support for blocks and lexical scopes to avoid namespace collisions and logically group blocks of code.

```asm
label: {
    identifier = 123;

    {
        identifier = identifier * 2; // shadows the parent identifier
        ld r0, identifier; // r0 = 246
    }

    ld r1, identifier; // r1 = 123
    add r1, r2; // r1 = 123 + 246
}

// ld r2, identifier; // error: cannot find identifier!
ld r2, label; // r2 = address of label
```

Blocks and lexical scopes are documented further in the [Scopes document](docs/assembler/scopes.md).

---

### Beautiful Error Messages

```asm
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

> [!WARNING]
> This example will not assemble.

![image](https://github.com/user-attachments/assets/bab51f30-499b-4016-8289-4002d5419d5a)

```asm
import * from "lib/print.asm";
print = 123;
```

> [!WARNING]
> This example will not assemble.

![image](https://github.com/user-attachments/assets/26d1c547-e1e1-45e0-a528-f46f0ffc836b)
