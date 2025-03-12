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
	or status, interupt_enable_bit; // enable interupts

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
	ld pc, end;

isr:
	// all registers need to be saved (except ilr)
	ld *sp.dec, lr;
	ld *sp.dec, status;
	ld *sp.dec, r0;
	ld *sp.dec, r1;

	ld r0, isr_string;
	ld *sp.dec, r0;
	ld pc.link, print;

	// check the interupt and acknowledge it
	ld r0, ic;
	and.t r0, timer_interupt_mask;
	ld.zc timer_ack, r0; // r0 doesn't really matter, just need to do a write

	ld r1, *sp.inc;
	ld r0, *sp.inc;
	ld status, *sp.inc;
	ld lr, *sp.inc;

	// enable interupts again (the cpu disabled interupts for us)
	or status, interupt_enable_bit;

	ld pc, ilr;

sw_isr:
	ld pc, ilr;

exception_handler:
	ld pc, ilr;

isr_string: "got an irq!\n\0";
