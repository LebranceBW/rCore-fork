use crate::kernel_info;
use crate::task::{run_next_task, terminal_current};

pub fn sys_exit(exit_code: i32) -> isize {
    kernel_info!("Application exited with code {}", exit_code);
    terminal_current();
    0
}

pub fn sys_yield() -> isize {
    run_next_task();
    0
}