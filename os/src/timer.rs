use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer_interrupt_reg;
use riscv::register::sie;
use riscv::register::time;

type TimeTick = usize;
type TimeMS = usize;
// type TimeS = usize;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

pub fn get_time() -> TimeTick {
    time::read()
}

pub fn get_time_ms() -> TimeMS {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn set_time_trigger() {
    set_timer_interrupt_reg(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub fn init() {
    unsafe { sie::set_stimer() };
    set_time_trigger();
}
