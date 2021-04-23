use crate::config::MAX_JOB_NUM;
use crate::task::context::Task;
use crate::task::context::TaskStatus;
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
    tasks: [Task; MAX_JOB_NUM],
    tasks_num: usize,
    current_task_id: Option<usize>,
}
pub struct TaskManager {
    inner: RefCell<TaskManagerInner>,
}

unsafe impl Sync for TaskManager {}

impl TaskManager {
    pub fn new((addr, tasks_num): ([usize; MAX_JOB_NUM], usize)) -> Self {
        let mut tasks: [Task; MAX_JOB_NUM] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..tasks_num {
            tasks[i] = Task::new(addr[i], i);
        }
        let current_task_id = None;
        Self {
            inner: RefCell::new(TaskManagerInner {
                tasks,
                tasks_num,
                current_task_id,
            }),
        }
    }
    pub fn run_next(&self) {
        if let Some(task_id) = self.find_a_ready_task() {
            unsafe {
                debug!("Task_{} switch in", task_id);
                let switch_in_context_ptr = {
                    let mut inner = self.inner.borrow_mut();
                    let in_task = &mut inner.tasks[task_id];
                    in_task.status = TaskStatus::Running;
                    in_task.ptr_to_context()
                };
                let switch_out_context_ptr = {
                    let mut inner = self.inner.borrow_mut();
                    let current_task_id = inner.current_task_id.unwrap();
                    inner.current_task_id = Some(task_id);
                    debug!("Task_{} switch out", current_task_id);
                    let out_task = &mut inner.tasks[current_task_id];
                    out_task.status = TaskStatus::Ready;
                    out_task.ptr_to_context()
                };
                __switch_task(switch_out_context_ptr, switch_in_context_ptr);
            }
        } else {
            panic!("All tasks finished");
        }
    }

    fn find_a_ready_task(&self) -> Option<usize> {
        let inner = self.inner.borrow_mut();
        for i in 0..inner.tasks_num {
            if inner.tasks[i].status == TaskStatus::Ready {
                return Some(i);
            }
        }
        inner.current_task_id
    }

    pub fn run_first(&self) {
        if let Some(task_id) = self.find_a_ready_task() {
            debug!("Task_{} start running!", task_id);
            let mut inner = self.inner.borrow_mut();
            let task = &mut inner.tasks[task_id];
            task.status = TaskStatus::Running;
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
        task_manager_inner.tasks[id].status = TaskStatus::Finished;
        task_manager_inner.current_task_id = None;
        drop(task_manager_inner);
        self.run_first();
    }
}
