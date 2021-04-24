use alloc::collections::BinaryHeap;
use crate::config::MAX_JOB_NUM;
use crate::task::context::{TIME_SLICE, TaskControlBlock};
use core::cell::RefCell;
use core::mem::drop;
use log::debug;
use log::warn;

const __UNUSED_TASK_CONTEXT_PTR: *mut usize = core::ptr::null_mut::<usize>();

global_asm!(include_str!("task.S"));

extern "C" {
    fn __switch_task(swap_out_task: *const usize, swap_in_task: *const usize);
}

struct TaskManagerInner {
    tasks: BinaryHeap<TaskControlBlock>,
    current_task: Option<TaskControlBlock>
}
pub struct TaskManager {
    inner: RefCell<TaskManagerInner>,
}

unsafe impl Sync for TaskManager {}

impl TaskManager {
    pub fn new((addr, tasks_num): ([usize; MAX_JOB_NUM], usize)) -> Self {
        let mut tasks = BinaryHeap::new();
        for i in 0..tasks_num{
            tasks.push(TaskControlBlock::new(addr[i], i))
        }
        Self {
            inner: RefCell::new(
                TaskManagerInner {
                    tasks,
                    current_task: None
                }
            )
        }
    }
    pub fn run_next(&self) {
        let mut inner = self.inner.borrow_mut();
        match (inner.current_task.take(), inner.tasks.pop()) {
            (Some(current_task), Some(mut next_task)) => {
                next_task.update_stride();
                let switch_in_context_ptr = next_task.ptr_to_context();
                let switch_out_context_ptr = current_task.ptr_to_context();
                debug!("Task_{} <==> Task_{}", current_task.task_id,  next_task.task_id);
                if current_task.total_time_ms > 500 * TIME_SLICE {
                    warn!("Current task time out. X");
                }
                else {
                    inner.tasks.push(current_task);
                }
                inner.current_task = Some(next_task);
                core::mem::drop(inner);
                unsafe {
                    __switch_task(switch_out_context_ptr, switch_in_context_ptr);
                }
            }
            (Some(mut task), None) => {
                if task.total_time_ms > 500 * TIME_SLICE {
                    warn!("Current task time out. X");
                    panic!("No more task need to execute");
                }
                task.update_stride();
                inner.current_task = Some(task);
                static mut hasLogged: bool = false;
                if unsafe { !hasLogged } {
                    debug!("No more tasks to switch in");
                    unsafe {
                        hasLogged = true;
                    }
                }
            }
            _ => panic!("No more task need to execute")
        }
    }

    fn find_a_ready_task(&self) -> Option<TaskControlBlock> {
        let mut inner = self.inner.borrow_mut();
        inner.tasks.pop()
    }

    pub fn run_first(&self) {
        if let Some(mut task) = self.find_a_ready_task() {
            debug!("Task_{} start running!", task.task_id);
            task.update_stride();
            let mut inner = self.inner.borrow_mut();
            let context_ptr = task.ptr_to_context();
            inner.current_task.replace(task);
            core::mem::drop(inner);
            unsafe {
                __switch_task(__UNUSED_TASK_CONTEXT_PTR, context_ptr);
            }
        } else {
            panic!("No tasks needs to be executed.");
        }
    }

    pub fn terminate_current(&self) {
        let mut task_manager_inner = self.inner.borrow_mut();
        let task = task_manager_inner.current_task.take().unwrap();
        debug!("Task_{} terminated!", task.task_id);
        drop(task_manager_inner);
        self.run_first();
    }

    pub fn set_priority(&self, prio: isize) -> isize{
        let mut inner = self.inner.borrow_mut();
        let mut task = inner.current_task.take().unwrap();
        let ret = task.set_priority(prio);
        inner.current_task = Some(task);
        ret
    }
}
