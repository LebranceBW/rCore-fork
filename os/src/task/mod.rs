pub mod context;
mod task_manager;

use crate::kernel_info;
use crate::loader::load_apps;
use lazy_static::*;
use task_manager::TaskManager;

lazy_static! {
    static ref TASK_MANAGER: TaskManager = {
        kernel_info!("Task manager initialized");
        TaskManager::new(load_apps())
    };
}

pub fn start_running() {
    TASK_MANAGER.run_first();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next();
}

pub fn terminate_current() {
    TASK_MANAGER.terminate_current();
}
