# Timer

Included in the project is a timer device, which is used to create interrupts at a set interval.

The timer is a countdown timer.

# Registers

The timer has the following registers:

| Register | Code | Purpose | Width |
|---|---|---|---|
| ACK | 0b00 | Writing will acknowledge an interrupt | 0 |
| CTRL | 0b01 | Used to control the timer (`START` and `CONTINUE` bits) | 2 |
| INTERVAL | 0b10 | What to reset the `TIMER` register to if the `CONTINUE` bit is set | 32 |
| TIMER | 0b11 | Used to set how many clock cycles until the timer produces an interrupt | 32 |

A more detailed description of each register is as follows:

## ACK Register

The `ACK` register does not contain any values, and can be thought more of a pseudo register. 

### Reading

Reading this register will result in 32-bits of 0s. 

### Writing

Writing to this register will cause the current interrupt to be acknowledged. If there is no interrupt then acknowledging the interrupt will do nothing.

The timer will keep it's `irq` line high until an interrupt is acknowledged.

## CTRL Register

The `CTRL` register contains flags to control the behaviour of the timer. The layout of this register is as follows:

| 1 | 0 |
|---|---|
| CONTINUE | START |

Setting the `START` bit will cause the timer to start. Starting will start decrementing the `TIMER` register until it reaches 0. After which, the timer will trigger an interrupt. This is called a "timeout".

If the `CONTINUE` bit is clear, then the `START` bit will be cleared after a timeout; this stops the timer from running again.

If the `CONTINUE` bit is set, then the `START` bit will not be cleared after a timeout, and the `TIMER` register will be reset to the `INTERVAL` register, and will start counting down again; this continues the timer after a timeout.

> [!NOTE]
> If the `CONTINUE` bit is set, then the timer never reaches 0: it will instead be loaded with the `INTERVAL` value. This ensures a consistent timeout period.

## INTERVAL Register

The `INTERVAL` register contains the value the `TIMER` register will be reset to when a timeout occurs and the `CONTINUE` bit is set.

## TIMER Register

The `TIMER` register contains the current value of the timer. The timer raises an interrupt when the `TIMER` register reaches 0. The `TIMER` register is set to the `INTERVAL` register when a timeout occurs and the `CONTINUE` bit is set.

## Memory Map

The simulator will map the timer to addresses `0x4000` to `0x4003`, as documented in the [Simulator document](simulator.md#memory-map).

Therefore, the registers are memory-mapped as follows:

| Register | Address |
|---|---|
| ACK | 0x4000 |
| CTRL | 0x4001 |
| INTERVAL | 0x4002 |
| TIMER | 0x4003 |

## Interupt Map

The simulator will set the timer as the 0th interrupt in the interrupt controller. The interrupt controller is documented in the [Interrupt Controller document](interrupt_controller.md).

## Examples

A complete working example can be found in the [Interrupts example](../../examples/interrupts.asm).

### Starting the timer to count 0x1000 clock cycles
```asm
timer_ctrl = *0x4001;
timer_timer = *0x4003;
timer_ctrl_start = 0b01;

// set up timer
// set timer interval to 0x1000 clock cycles
ld r0, 0x1000;
ld timer_timer, r0;

// start timer
ld r0, timer_ctrl_start;
ld timer_ctrl, r0;
```

### Starting the timer to count 0x1000 clock cycles and continue after timing out
```asm
timer_ctrl = *0x4001;
timer_interval = *0x4002;
timer_timer = *0x4003;
timer_ctrl_start = 0b01;
timer_ctrl_continue = 0b10;

// set up timer
// set timer interval to 0x1000 clock cycles
ld r0, 0x1000;
ld timer_interval, r0;
ld timer_timer, r0;

// start timer
ld r0, timer_ctrl_start | timer_ctrl_continue;
ld timer_ctrl, r0;
```

### Reading current `TIMER` value
```asm
timer_timer = *0x4003;
ld r0, timer_timer;
```

### Acknowledging an interrupt
```asm
timer_ack = *0x4000;
timer_interupt_mask = 0x0001;
ic = *0x4004;

ld r0, ic;
and.t r0, timer_interupt_mask;
ld.zc timer_ack, r0; // r0 doesn't really matter, just need to do a write
```
