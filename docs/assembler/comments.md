# Comments

The assembler allows comments similar to C style comments.

## Single-line Comments

A single-line comment can be created by writing `//`. A newline character will end the comment.

For example:

```c
add r1, r2; // this is an inline comment!
sub // mnemonic
r1, // operand 1
r2; // operand 2
```

## Multi-line Comments

A multi-line comment can be created by delimiting the comment with `/*` and `*/`.

For example:

```c
/*
this program does the following:
r1 = r1 + r2;
r1 = r1 - r2;
*/
add /* here are the operands: */ r1, r2;
sub r1, r2;
```

Multi-line comments **can** nest, so this will assemble with no issues:

```rs
/*
this is a comment /* and this is a nested comment */
this is some more comment
*/
add r1, r2;
sub r1, r2;
```
