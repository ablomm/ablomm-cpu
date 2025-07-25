# Assembly-time Assignments

The assembler supports assembly-time assignments, which are just a convenience for storing the result of an assembly-time expression, as detailed in the [Expressions document](expressions.md).

An assignment has two parts: an identifier, and an expression, separated by an equals sign.

Any value that may find itself in the operand of an instruction may be in the expression part of an assignment; both are expressions.

An example of such an assignment is as follows:

```c
value = 123 * 2;
```

You may also use identifiers in the expression part of another assignment:

```c
value = 123 * 2;
value2 = value * (12 >> 2) / 3;
```

Assignments can also act much like defines, where it aliases expressions involving registers. For example:

```c
value = r1;
value2 = *(r1 + 12 * 2);
```

Expressions which contain registers are not evaluated, but instead are passed to the operands of an instruction.

For example, consider the following example:

```c
value = r1;
value2 = *(r1 + 12 * 2);
ld value, value2;
```

This is equivalent to the following:

```c
ld r1, *(r1 + 24);
```

The location an assignment is defined does not affect the result; expressions do not evaluate registers.

For example, consider the following:

```c
ld r1, 123;
value = r1;
ld r1, 321;
ld r0, value;
```

This is equivalent to:

```c
ld r1, 123;
ld r1, 321;
ld r0, r1;
```

This means `r0` will contain the value `321` after it is ran **NOT** the value `123`; expressions do not evaluate registers.

## Hoisting

All identifiers are hoisted in their scope; this allows for using labels before they were defined.

For example, this is legal:

```c
ld r0, value;
value = 123;
```

## Shadowing

Block scopes (as detailed in the [Scopes document](scopes.md)) allow for shadowing identifiers.

An example of shadowing an identifier is as shown:

```c
value = 123;
{
    value = value * 2; // shadowing the identifier defined above
    ld r0, value;
}
ld r1, value;
```

This is equivalent to the following:

```c
ld r0, 246;
ld r1, 123;
```
