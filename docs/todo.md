# Todo

This document is a list of possible future features and bug fixes.

## CPU

- [ ] Make memory clocked:  
Currently, memory is not clocked, but most memories seem to be clocked, so to make synthesis easier memory should be clocked.

- [ ] PUSH and POP support multiple registers:  
Currently PUSH and POP only works on one register at a time.  
E.g.: `push r1, r2, r3;`

- [ ] Floating Point Arithmetic Unit:  
Have a hardware-supported way to perform floating point operations.

## Assembler

- [ ] Progress bar:  
Have a progress bar for the progress of the compilation.

- [ ] Have instructions as expressions:  
Currently instructions are handled in a special way, but it may be possible to consider an instruction as simply an expression that results in a u32 number.  
E.g.: `value = (add.s r1, r2) * 2;`.  
This would also mean something like `add.s r1, r2;` is just a gen literal.  
Although, this would make variable length instructions much harder to implement.

- [ ] Statically determined `pc`:  
It is possible to determine the value of `pc` statically at compile time. Perhaps have an identifier such as `$pc` that evaluates to the value of the current address at compile time.  
This would allow things like `add pc, label - $pc;` which is a jump using offsets instead of absolute addresses.

- [ ] Symbol table `get_recursive()` returns borrowed value instead of a cloned one  
Currently `get_recursive()` returns a cloned value, but it might be possible to return some ref object.  
I tried this for a while, but couldn't figure out how to make it work with the borrow checker, but I think it might be possible.

- [ ] Efficient symbol table inserts:  
Currently, the assembler will do 3 passes to fill the symbol tables, but for the second and third pass, it re-fills a bunch of symbols when it doesn't need to.
Also consider adding a sensitivity list for each symbol to recursively evaluate expressions for the symbol table.

- [ ] Circular imports:  
Currently, circular imports is not supported.

- [ ] Multithreading:  
Currently, the assembler is single-threaded.

- [ ] Floating points:  
Allow writing decimal values that are converted to floating points.

- [ ] Tests:  
There is no test currently for the assembler.
