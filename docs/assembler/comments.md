# Comments

The assembler allows comments similar to C style comments.

## Single-line Comments

A single-line comment can be created by writing `//`. A newline character will end the comment.

For example:

```c
add r1, r2; // this is an inline comment!
sub r1, r2;
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
add r1, r2;
sub r1, r2;
```

Multi-line comments cannot nest, so this will **NOT** assemble:

```c
/*
this is a comment /* nested comment */
*/
add r1, r2;
sub r1, r2;
```

> [!WARNING]
> This example will not assemble.

## Limitations

Currently, comments must come after a complete statement, and cannot come between tokens.

For example, the following example will **NOT** assemble:

```c
add /* comment */ r1, r2;
add //comment
r1, r2;
```

> [!WARNING]
> This example will not assemble.

> [!NOTE]
> In the future this may be changed.
