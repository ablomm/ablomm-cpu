# Interrupt Controller

The interrupt controller is a very basic interrupt controller which allows multiple devices to be connected to a single `IRQ` line.

As input, the interrupt controller will receive 16 devices IRQs.

As output, the interrupt controller will send an IRQ if any of the inputted IRQs are high.

Reading from the interrupt controller will result in a 16-bit value. This value is simply the 16 IRQ inputs, and allow us to determine which device caused an interrupt.

Writing does nothing.

## Memory Map

The simulator will map the interrupt controller to address `0x4004`, as documented in the [Simulator document](simulator.md#memory-map).

## Examples

### Reading the interrupt controller

```asm
ic = *0x4004;
ld r0, ic;
```
