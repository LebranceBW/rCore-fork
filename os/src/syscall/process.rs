use crate::task::{run_next_task, set_priority, terminate_current};
use crate::timer::get_time_ms;
use log::info;

pub fn sys_exit(exit_code: i32) -> isize {
    info!("Application exited with code {}", exit_code);
    terminate_current();
    0
}

pub fn sys_yield() -> isize {
    run_next_task();
    0
}

pub fn sys_get_time() -> isize {
    unsafe { get_time_ms() as isize }
}

pub fn sys_set_priority(prio: usize) -> isize {
    set_priority(prio);
    0
}
