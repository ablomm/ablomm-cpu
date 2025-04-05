# Unintuitive Errors

Some assembler errors are a bit unintuitive, such errors are described here:

## Unknown value of expression

This error is a bit complicated, and is caused when an expression is dependent on a future address (label), but the value of the expression would effect that address (label).

If such a dependency causes an expression to be unknowable at a point were it must be knowable, then this error will appear.

In general, if there is an expression value `x` such that `x = f(x)` where `f(x) != x`, then the error will appear.

### Examples

#### Future label string concatenation
Consider the following example:

```asm
"" + label;
label:
```
> [!WARNING]
> This example will not assemble.

You may be surprised to know that this example causes the error `Unknown value of expression`.

This is because the value of `label` is dependent on the value of `"" + label`.

In theory, this value should be computable using the following calculation:

$`label=\left\lceil log_{10}\left(\left\lceil \frac{label}{4} \right\rceil + 1 \right) \right\rceil`$

If you solve this equation, you find that `label = 1` is a solution.

But, in general, it is not possible to solve these types of dependencies, as there may be no solutions or multiple solutions.

The assembler does not try to solve it, and instead just gives up.

> [!NOTE]
> Imported files are ordered after the importer, as described in the [Imports and Exports document](imports-and-exports.md#imports-structure-in-machine-code), which means, for all intents and purposes, an imported identifier is defined after all statements in the importer, regardless of where the import statement appears.
