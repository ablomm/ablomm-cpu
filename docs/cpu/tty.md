# TTY device

Included in the simulator is a TTY device, which allows printing of UTF-8 characters onto the terminal.

Writing to the TTY device will cause the 8 LSBs to be printed to the terminal. These 8 bits are interpreted as UTF-8 values.

Reading is not supported, and may cause any random value to be read.

## Memory Map

The simulator will map the TTY to address `0x4006`, as documented in the [Simulator document](simulator.md#memory-map).

## Examples

### Printing the letter `A`

```c
tty = *0x4006;
ld r0, 'A';
ld tty, r0;
ld r0, '\n';
ld tty, r0;
```
