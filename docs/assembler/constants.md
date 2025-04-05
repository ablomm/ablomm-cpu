# Compile Time Constants

The assembler supports compile time constants, which are just a convinence for storing the result of an expression, as detailed in the [Expressions document](expressions.md).

An example of such a constant is as follows:

```asm
value = 123 * 2;
```

You may also use constants in the expressions of another constant assignment:

```asm
value = 123 * 2;
value2 = value * (12 >> 2) / 3;
```

This is allowed since the value of each constant can be computed at compile time.

Constants can also act much like defines, where it aliases expressions involving registers. For example:

```asm
value = r1;
value2 = *(r1 + 12 * 2);
```

Any value that may find itself in the operand of an instruction may be assigned to a constant.

Constants which contain registers are not evaluated, but instead are passed to the operands of an instruction.

for example, consider the following example:

``` asm
value = r1;
value2 = *(r1 + 12 * 2);
ld value, value2;
```

This is equivalent to the following:

``` asm
ld r1, *(r1 + 24);
```

The location a constant is defined does not affect the result; constant's do not evaluate registers.

For example, consider the following:

``` asm
ld r1, 123;
value = r1;
ld r1, 321;
ld r0, value;
```

this is equiavlent to:

``` asm
ld r1 123;
ld r1, 321;
ld r0, r1;
```

This means `r0` will contain the value `321` after it is ran **NOT** the value `123`; registers are not evaluated.

## Hoisting

All identifiers are hoisted in their scope; this allows for using labels before they were defined.

For example, this is legal:

``` asm
ld r0, value;
value = 123;
```

## Shadowing

Constants cannot be shadowed the usual way since they are hoisted. Instead, you need to use a block scope, as detailed in the [Scopes document](scopes.md).

An example of shadowing a constant is as shown:

``` asm
value = 123;
{
    value = value * 2; // shadowing the constant value defined above
    ld r0, value;
}
ld r1, value;
```

This is equivalent to the following:

``` asm
ld r0, 246;
ld r1, 123;
```
