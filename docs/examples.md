# Examples

There are various examples located in the [examples directory](../examples/).

## Calling Convention

The examples use the following calling convention:

### Inputs

All inputs are pushed to the stack. The callee will pop these before it returns to the caller.

### Outputs

All outputs are returned in `r0` and `r1`. If more than 2 words are needed, then the caller will supply as input a pointer for the callee to write to.

### Saved registers

All registers except `r0`, `r1` are saved by the callee. This includes saving `status`, `lr`, and `pc` (by returning to the caller). `sp` will be incremented by the number of inputs.
