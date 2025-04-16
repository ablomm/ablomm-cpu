// status register bitmasks

// NZCV bits
export negative_bit = 1 << 5;
export carry_bit = 1 << 3;
export overflow_bit = 1 << 2;
export zero_bit = 1 << 4;

export interupt_enable_bit = 1 << 1; // the bit in the status register that controls if interrupts are enabled
export supervisor_mode_bit = 1; // the bit in the status register that controls supervisor mode

// timer
export timer_ack = *0x4000; // timer acknowledge register
export timer_ctrl = *0x4001; // timer control register
export timer_interval = *0x4002; // timer interval register
export timer_timer = *0x4003; // timer timer register

export timer_ctrl_start = 0b01; // bit mask to write to the timer control register to start the timer
export timer_ctrl_continue = 0b10; // bit mask to write to the timer control register to continue the timer

// interrupt controller
export ic = *0x4004; // the address of the interrupt controller memory mapped io device

// interrupt masks
export timer_interupt_mask = 0x0001; // mask ic with this to see if the timer created an interrupt

// power controller
export power = *0x4005; // the address of the power controller
export power_shutdown_code = 0; // write this to power to shutdown
export power_restart_code = 1; // write this to power to restart

// tty
export tty = *0x4006; // address of the terminal memory mapped io device
