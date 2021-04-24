use super::dispatcher::{Dispatcher, JustEnoughOne, Stride};
use crate::config::MAX_JOB_NUM;
use crate::task::context::TaskControlBlock;
use core::cell::RefCell;
use core::mem::drop;
use core::mem::MaybeUninit;
use log::debug;

const __UNUSED_TASK_CONTEXT_PTR: *mut usize = core::ptr::null_mut::<usize>();

global_asm!(include_str!("task.S"));

extern "C" {
    fn __switch_task(swap_out_task: *const usize, swap_in_task: *const usize);
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_JOB_NUM],
    tasks_num: usize,
    current_task_id: Option<usize>,
    // dispatcher: Stride,
    dispatcher: JustEnoughOne
}
pub struct TaskManager {
    inner: RefCell<TaskManagerInner>,
}

unsafe impl Sync for TaskManager {}

impl TaskManager {
    pub fn new((addr, tasks_num): ([usize; MAX_JOB_NUM], usize)) -> Self {
        let mut tasks: [TaskControlBlock; MAX_JOB_NUM] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..tasks_num {
            tasks[i] = TaskControlBlock::new(addr[i], i);
        }
        let current_task_id = None;
        Self {
            inner: RefCell::new(TaskManagerInner {
                tasks,
                tasks_num,
                current_task_id,
                dispatcher: JustEnoughOne::new(tasks_num),
            }),
        }
    }
    pub fn run_next(&self) {
        let current_task_id = {
            let inner = self.inner.borrow_mut();
            inner.current_task_id
        };
        let task_id = self.find_a_ready_task();
        match (current_task_id, task_id) {
            (Some(current_task_id), Some(next_task_id)) => {
                debug!("Task_{} switch in", next_task_id);
                let switch_in_context_ptr = {
                    let mut inner = self.inner.borrow_mut();
                    let in_task = &mut inner.tasks[next_task_id];
                    in_task.ptr_to_context()
                };
                let switch_out_context_ptr = {
                    let mut inner = self.inner.borrow_mut();
                    inner.dispatcher.push(current_task_id);
                    inner.current_task_id = Some(next_task_id);
                    debug!("Task_{} switch out", current_task_id);
                    let out_task = &mut inner.tasks[current_task_id];
                    out_task.ptr_to_context()
                };
                unsafe {
                    __switch_task(switch_out_context_ptr, switch_in_context_ptr);
                }
            }
            (Some(_), None) => {
                static mut hasLogged: bool = false;
                if unsafe { !hasLogged } {
                    debug!("No more tasks to switch in");
                    unsafe {
                        hasLogged = true;
                    }
                }
            }
            _ => {
                panic!("All tasks finished");
            }
        }
    }

    fn find_a_ready_task(&self) -> Option<usize> {
        let mut inner = self.inner.borrow_mut();
        inner.dispatcher.pop()
    }

    pub fn run_first(&self) {
        if let Some(task_id) = self.find_a_ready_task() {
            debug!("Task_{} start running!", task_id);
            let mut inner = self.inner.borrow_mut();
            let task = &mut inner.tasks[task_id];
            let context_ptr = task.ptr_to_context();
            inner.current_task_id = Some(task_id);
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
        let id = task_manager_inner.current_task_id.unwrap();
        debug!("Task_{} terminated!", id);
        task_manager_inner.current_task_id = None;
        drop(task_manager_inner);
        self.run_first();
    }

    pub fn set_priority(&self, prio: usize) {
        let mut inner = self.inner.borrow_mut();
        let id = inner.current_task_id.unwrap();
        inner.dispatcher.set_priority(id, prio);
    }
}
