# Memory

The memory for the CPU is a bit special in that each address holds 32 bits, rather than the typical 8 bits.

Reads are asynchronous (although this may be changed in the future for better compatibility).

Writes are synchronous.

## Read Only Memory

The simulator will create a block of ROM from address `0x0000` to `0x3fff` inclusive, as documented in the [Simulator document](simulator.md#memory-map).

Writing to ROM does nothing.

ROM contains the machine code.

## Random Access Memory

The simulator will create a block of RAM from addresses `0x8000` to `0xffff` inclusive, as documented in the [Simulator document](simulator.md#memory-map).
