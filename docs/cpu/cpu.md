# CPU

![cpu](https://github.com/user-attachments/assets/e9c9861d-8890-4763-a765-1c42a2c68891)

Ablomm CPU is a 32-bit CPU.

## Ports

| Name | Description | Width |
|---|---|---|
| data | The data bus; memory outputs on this bus for reads. Memory uses this bus for writes. | 32 |
| addr | The address bus; memory uses this bus to address reads and writes. | 32 |
| rd | Signal to read from memory | 1 |
| wr| Signal to write to memory | 1 |
| irq | Interrupt request; causes a hardware interrupt as outlined in [ISA document](isa.md#interrupt-vector-table) | 1 |
| rst | Reset; causes the CPU to reset all registers as outlined in the [ISA document](isa.md#interrupt-vector-table) | 1 |
| en | Enable; starts the CPU when set, pauses the CPU when clear. Takes affect on the next falling edge of the clock | 1 |
| clk | Clock | 1 |

## Starting

The CPU will power on in a disabled state. To enable, simply set the `en` port. On the next falling edge, the CPU will begin executing. This also applies to resets.

## Stopping

The CPU can be paused by clearing the `en` port. On the next falling edge, the CPU will pause execution.
