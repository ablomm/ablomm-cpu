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

```c
export tty = *0x4006; // the address of the terminal memory mapped io device
export power = *0x4005; // the address of the power memory mapped io device
export power_shutdown_code = 0; // write this to power to shutdown
export power_restart_code = 1; // write this to power to restart
```

### Count from 0 to 9 and print it to the terminal:

```c
/*
prints 0 to 9 to the tty
*/

import * from "lib/defines.asm";

    num = r0; // alias num to r0
    new_line = r1; // alias new_line to r1

    ld num, '0'; // load number with the ascii of 0
loop:
    ld tty, num; // print the ascii num to the terminal
    add num, 1; // get next ascii character
    sub.t num, '9'; // test if the number is ascii 9
    ld.ule pc, loop; // if previous test is less than or equal (unsigned), then loop

    ld new_line, '\n'; // load a newline character
    ld tty, new_line; // print the newline character

    // shutdown the cpu
    ld r0, power_shutdown_code;
    ld power, r0;
```

![image](https://github.com/user-attachments/assets/d6a8093b-0f3b-4abb-8116-9a1a80520f6d)

### Print a null terminated string to the terminal:

```c
import * from "defines.asm";

// inputs: string to print
export print: {
        // setup stack frame
        push fp;
        ld fp, sp;

        // saved registers
        push status;
        push r2;

        string_ptr_in = *(fp + 1); // alias for the string on the input stack

        string_ptr = r0; // the pointer to the string, in a register (rather than on the stack)
        string_word = r1; // holds a word of string characters to print (4 characters per word)
        bytes_left = r2; // how many bytes in the word is left to print

        ld string_ptr, string_ptr_in; // get the input and put it in a register

    // print every character in the string_word
    print_word:
        ld string_word, *string_ptr; // get the word string_ptr is pointing at
        ld bytes_left, 4; // 4 bytes in a word

    /* 
    since memory is only word addressable
    we need to do some rotates to get each byte
    individually
    */

    // print a single byte from the string_word
    print_byte:
        rol string_word, 8; // get the most significant byte on the lower 8 bits (to print it in the correct order)
        and.t string_word, 0xff; // test the byte to print
        ld.zs pc, return; // i.e. lsb is null '\0' then return (we are done)
        ld tty, string_word; // print the least significant byte
        sub.s bytes_left, 1; // subtract bytes_left and set the status flags
        ld.ne pc, print_byte; // if bytes_left didn't equal 1 before the subtraction, then print the next byte

        // we have printed all the bytes in the word
        add string_ptr, 1; // get the next word in the string
        ld pc, print_word; // branch to print the word

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

```c
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

```c
export print: {
  ...
}
```

`hello_world.asm`:

```c
import print from "lib/print.asm";

    ld r0, string;
    ld pc.link, print;

string: "hello world!\n\0";
```

File imports are documented further in the [Imports and Exports document](docs/assembler/imports-and-exports.md).

---

### Compile Time Expressions

The assembler has support for compile time assignments and expressions similar to c++'s constexpr:

```c
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

```c
label: {
    identifier = 123;

    {
        identifier = identifier * 2; // shadows the parent identifier
        ld r0, identifier; // r0 = 246
    }

    ld r1, identifier; // r1 = 123
    add r0, r1; // r0 = 246 + 123
}

// ld r2, identifier; // error: cannot find identifier!
ld r2, label; // r2 = address of label
```

Blocks and lexical scopes are documented further in the [Scopes document](docs/assembler/scopes.md).

---

### Beautiful Error Messages

```c
label: {
    identifier = 123;

    {
        identifier = identifier * 2; // shadows the parent identifier
        ld r0, identifier; // r0 = 246
    }

    ld r1, identifier; // r1 = 123
    add r0, r1; // r0 = 246 + 123
}

ld r2, identifier; // error: cannot find identifier!
ld r2, label; // r2 = address of label
```

> [!WARNING]
> This example will not assemble.

![image](https://github.com/user-attachments/assets/bab51f30-499b-4016-8289-4002d5419d5a)

```c
import * from "lib/print.asm";
print = 123;
```

> [!WARNING]
> This example will not assemble.

![image](https://github.com/user-attachments/assets/26d1c547-e1e1-45e0-a528-f46f0ffc836b)
