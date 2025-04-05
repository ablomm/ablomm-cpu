# Labels

Labels work just the same as most other assemblers; they are simply identifiers with the value of the address they point to.  

For example:

```asm
    ld pc, label;
    ld r1, 321;
loop:
    ld pc, loop;

label:
    ld r1, 123;
    ld pc, loop;
```

In this example, `r1` will have the value `123` when it reaches the loop. This is because the program jumps to the label `label`.

Labels can be used in the same way as any other identier, and can therefore be used in the same way as shown in the [Assignments document](assignments.md).

For example, labels can be part of an expression:

```asm
    ld pc, label + 1;
    ld r1, 321;
loop:
    ld pc, loop;

label:
    ld r1, 123;
    ld pc, loop;
```

This example now causes `r1` to never be set because it skips the first instruction after `label` and instead jumps straight to `ld pc, loop;`.

In both of these examples the value of `label` is the number `3`, and the value of `loop` is the number `2`. This is because the address of the first instruction is 0.

Therefore, we can write an equivalent program to the first example without labels:

```asm
ld pc, 3;
ld r1, 321;
ld pc, 2;
ld r1, 123;
ld pc, 2;
```
