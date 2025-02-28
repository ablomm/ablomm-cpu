# Power Controller

Included in the simulator is a power controller device, which allows terminating the simulator or reseting the CPU.

Writing the value 0 to the power controller will terminate the simulator.

Writing the value 1 to the power controller will reset the CPU.

> [!NOTE]
> Resetting the CPU will only cause the CPU registers to reset. Memory and devices are not reset.

Writing any other value does nothing.

Reading is not supported, and may cause any random value to be read.

## Memory Map

The simulator will map the power controller to address `0x4005`.

## Examples

### Terminating the simulator

```asm
power = *0x4005;
power_shutdown_code = 0;
  ld r0, power_shutdown_code;
  ld power, r0;
```

### Reseting the CPU

```asm
power = *0x4005;
power_restart_code = 1;
  ld r0, power_restart_code;
  ld power, r0;
```
