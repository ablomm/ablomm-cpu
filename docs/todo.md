# Todo

This document is a list of possible future features.

## CPU

### Make memory clocked

Currently, memory is not clocked, but most memories seem to be clocked, so to make synthesis easier memory should be clocked.

### PUSH and POP support multiple registers

Currently PUSH and POP only works on one register at a time.

E.g.: `push r1, r2, r3;`

## Assembler

### Progress bar

Have a progress bar for the progress of the complication.

### Have instructions as expressions

Currently instructions are handled in a special way, but it may be possible to consider an instruction as simply an expression that results in a u32 number.

E.g.: `value = (add.s r1, r2) * 2;`

This would also mean something like `add.s r1, r2;` is just a gen literal.

Although, this would make variable length instructions much harder to implement.

### Statically determined pc

It is possible to determine the value of pc statically at compile time. Perhaps have an identifier such as $pc that evaluates to the value of the current address at compile time.

This would allow things like `add pc, label - $pc;` which is a jump using offsets instead of absolute addresses.

### Symbol table returns ref for get_recursive()

Currently get_recursive returns a cloned value, but it might be possible to return some ref object.

I tried this for a while, but couldn't figure out how to make it work with the borrow checker, but I think it might be possible.

### Tests

There is no test currently for the assembler.
