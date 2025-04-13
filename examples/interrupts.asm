/*
sets up the interupt vector table, and initalises the timer to create an interrupt
*/

import * from "lib/defines.asm";
import print from "lib/print.asm";

	// interrupt vector table
	ld pc, start;
	ld pc, isr;
	ld pc, sw_isr;
	ld pc, exception_handler;

start:
	int; // software interrupt!
	0x0e000000; // invalid instruction -- exception!

	or status, interupt_enable_bit; // enable hardware interrupts

	// set up timer
	// set timer interval to 0x1000 clock cycles
	ld r0, 0x1000;
	ld timer_interval, r0; // only needed if timer_ctrl_continue
	ld timer_timer, r0;

	// start timer
	ld r0, timer_ctrl_start;

	// use this one if you want the timer to continue counting after timing out!
	// ld r0, timer_ctrl_start | timer_ctrl_continue;

	ld timer_ctrl, r0;

end:
	// loop while we wait for a hardware interrupt
	ld pc, end;

isr:
	push lr;
	push r0;
	push r1;

	ld r0, hwint_string;
	push r0;
	ld pc.link, print;

	// check the interupt and acknowledge it
	ld r0, ic;
	and.t r0, timer_interupt_mask;
	ld.zc timer_ack, r0; // r0 doesn't really matter, just need to do a write

	pop r1;
	pop r0;
	pop lr;

	// the cpu pushed status and pc for us, we just need to pop them!
	pop status;
	pop pc;

sw_isr:
	push lr;
	push r0;
	push r1;
	
	ld r0, swint_string;
	push r0;
	ld pc.link, print;

	pop r1;
	pop r0;
	pop lr;

	// the cpu pushed status and pc for us, we just need to pop them!
	pop status;
	pop pc;

exception_handler:
	push lr;
	push r0;
	push r1;
	
	ld r0, except_string;
	push r0;
	ld pc.link, print;

	pop r1;
	pop r0;
	pop lr;

	// the cpu pushed status and pc for us, we just need to pop them!
	pop status;
	pop pc;

hwint_string: "got a hardware interrupt!\n\0";
swint_string: "got a software interrupt!\n\0";
except_string: "got an exception!\n\0";
