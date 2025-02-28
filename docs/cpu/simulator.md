# Simulator

The simulator will simulate the CPU along with some devices.

The simulator can be ran, as outlined in the [Setup document](../setup.md#simulate).

The simulator allowing passing in a plusargs `+src=<FILE>` which contains the machine code file to read into ROM.

## Memory Map

The memory map of the simulator is as follows:

| Device | Address Span (inclusive) |
|---|---|
| [ROM](memory.md#read-only-memory) | `0x0000` to `0x3fff` |
| [Timer](timer.md) | `0x4000` to `0x4003` |
| [Interrupt controller](interrupt_controller.md) | `0x4004` |
| [Power controller](power_controller.md) | `0x4005` |
| [TTY](tty.md) | `0x4006` |
| [RAM](memory.md#random-access-memory) | `0x8000` to `0xffff` |

More information for each device can be found in their respective documentation.
