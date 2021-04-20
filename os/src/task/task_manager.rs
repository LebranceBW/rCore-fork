use crate::config::MAX_JOB_NUM;
use core::mem::drop;
use crate::kernel_info;
use crate::kernel_debug;
use crate::task::context::Task;
use crate::task::context::TaskContext;
use crate::task::context::TaskStatus;
use core::cell::RefCell;
use core::mem::MaybeUninit;
use lazy_static::*;

global_asm!(include_str!("task.S"));

extern "C" {
    fn __switch_task(swap_out_task: *const usize, swap_in_task: *const usize);
}

struct TaskManagerInner {
    tasks: [Task; MAX_JOB_NUM],
    task_num: usize,
    current_task_id: Option<usize>
}
pub struct TaskManager {
    inner: RefCell<TaskManagerInner>,
}

unsafe impl Sync for TaskManager {}

impl TaskManager {
    pub fn new((addr, task_num): ([usize; MAX_JOB_NUM], usize)) -> Self {
        let mut tasks: [Task; MAX_JOB_NUM] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..task_num {
            tasks[i] = Task::new(addr[i], i);
        }
        let current_task_id = None;
        Self {
            inner: RefCell::new(TaskManagerInner { tasks, task_num, current_task_id }),
        }
    }
    pub fn run_next(&self) {
        if let Some(context_ptr) = self.search_a_ready_task() {
            unsafe {
                let mut inner_task_manager = self.inner.borrow_mut();
                let task_id = inner_task_manager.current_task_id.unwrap();
                inner_task_manager.tasks[task_id].mark_status(TaskStatus::Ready);
                let task_context = inner_task_manager.tasks[task_id].ptr_to_context();
                drop(inner_task_manager);
                __switch_task(task_context, context_ptr);
            }
        } else {
            panic!("All tasks finished");
        }
    }

    fn search_a_ready_task(&self) -> Option<*const usize> {
        let mut inner_task_manager = self.inner.borrow_mut();
        for i in 0..inner_task_manager.task_num {
            if inner_task_manager.tasks[i].status == TaskStatus::Ready {
                inner_task_manager.current_task_id = Some(i);
                inner_task_manager.tasks[i].mark_status(TaskStatus::Running);
                let ptr = inner_task_manager.tasks[i].ptr_to_context();
                return Some(ptr);
            }
        }
        None
    }

    pub fn run_first(&self) {
        if let Some(context_ptr) = self.search_a_ready_task() {
            unsafe {
                __switch_task(0 as *mut usize, context_ptr);
            }
        } else {
            panic!("No tasks needs to be executed.");
        }
    }

    pub fn terminal_current(&self) {
        let mut task_manager_inner = self.inner.borrow_mut();
        let id = task_manager_inner.current_task_id.unwrap();
        task_manager_inner.tasks[id].status = TaskStatus::Finished;
        drop(task_manager_inner);
        self.run_first();
    }
}
