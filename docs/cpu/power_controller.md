# Power Controller

Included in the simulator is a power controller device, which allows terminating the simulator or resetting the CPU.

Writing the value `0` to the power controller will terminate the simulator.

Writing the value `1` to the power controller will reset the CPU. Resetting is documented in the [ISA document](isa.md#interrupt-vector-table).

> [!NOTE]
> Resetting the CPU will only cause the CPU registers to reset. Memory and devices are not reset.

Writing any other value does nothing.

Reading is not supported, and may cause any random value to be read.

## Memory Map

The simulator will map the power controller to address `0x4005`, as documented in the [Simulator document](simulator.md#memory-map).

## Examples

### Terminating the simulator

```c
power = *0x4005;
power_shutdown_code = 0;
ld r0, power_shutdown_code;
ld power, r0;
```

### Resetting the CPU

```c
power = *0x4005;
power_restart_code = 1;
ld r0, power_restart_code;
ld power, r0;
```
