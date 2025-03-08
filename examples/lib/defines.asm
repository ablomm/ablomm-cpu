// status register bitmasks
export supervisor_mode_bit = 1;
export interupt_enable_bit = 1 << 1;
export overflow_bit = 1 << 2;
export carry_bit = 1 << 3;
export zero_bit = 1 << 4;
export negative_bit = 1 << 5;

// interrupt masks
export timer_interupt_mask = 0x0001;

// timer
export timer_ack = *0x4000;
export timer_ctrl = *0x4001;
export timer_interval = *0x4002;
export timer_timer = *0x4003;
export timer_ctrl_start = 0b01;
export timer_ctrl_continue = 0b10;

// interrupt controller
export ic = *0x4004;

// power controller
export power = *0x4005;
export power_shutdown_code = 0;
export power_restart_code = 1;

// tty
export tty = *0x4006;

