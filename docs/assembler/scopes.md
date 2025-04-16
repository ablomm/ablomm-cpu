# Scopes

The assembler supports lexical scopes. Each file has it's own scope, and each block within a file contains it's own scope. A file for our purposes can be considered just another block.

A scope is defined to be the span in which an identifier is valid.

## Blocks

A block is simply a list of statements delimited by `{` and `}`. This is quite similar to C blocks.

A block can reference identifiers defined in it's containing block.

For example, this results in `r0` containing the value `123`:

```c
value = 123;
{
    ld r0, value;
}

```

An identifier defined in a child block cannot be referenced from the parent block.

For example, this **DOES NOT ASSEMBLE**:

```c
{
    value = 123;
}
ld r0, value;
```

> [!WARNING]
> This example will not assemble.

> [!NOTE]
> There is a mechanism called block exports that allows a value to be referenced from the parent scope, as detailed in the [Imports and Exports document](imports-and-exports.md#block-exports).

### Block Scoped Imports

Blocks can contain import statements, in which case the imported identifiers are only valid within that block.

For example:

```c
{
    import print from "lib/print.asm";
  
        ld r0, string;
        push r0;
        ld pc.link, print;

    end:
        ld pc, end;
}
// cannot reference print from this scope!

string: "hello_world!\n\0";
```
