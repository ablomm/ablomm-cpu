import * from "lib/defines.asm";
import print from "lib/print.asm";

	ld pc, start;
	ld pc, isr;
	ld pc, sw_isr;
	ld pc, exception_handler;

start:
	or status, interupt_enable_bit; // enable interupts

	// set up timer
	// set timer interval to 0x1000 clock cycles
	ld r0, 0x1000;
	ld timer_interval, r0;

	// start timer
	ld r0, timer_ctrl_start;

	// use this one if you want the timer to continue counting after timing out!
	// ld r0, timer_ctrl_start | timer_ctrl_continue;

	ld timer_ctrl, r0;

end:
	ld pc, end;

isr:
	push lr;
	push status;
	push r0;

	ld r0, isr_string;
	ld pc.link, print;

	// check the interupt and acknowledge it
	ld r0, ic;
	and.t r0, timer_interupt_mask;
	ld.zc timer_ack, r0; // r0 doesn't really matter, just need to do a write

	pop r0;
	pop status;
	pop lr;

	// enable interupts again (the cpu disabled interupts for us)
	or status, interupt_enable_bit;

	// the cpu pushed pc for us, we just need to pop it
	pop pc;

sw_isr:
	pop pc;

exception_handler:
	pop pc;

isr_string: "got an irq!\n\0";
