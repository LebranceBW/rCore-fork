use crate::config::*;
use crate::kernel_debug;
use crate::trap::TrapContext;
use core::default::Default;
use core::mem::size_of;

#[repr(C)]
#[derive(Debug)]
pub struct TaskContext {
    pub ra: usize,
    pub s: [usize; 12],
}

unsafe impl Sync for TaskContext {}

impl TaskContext {
    pub fn app_init_task_context() -> Self {
        // return to __restore
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            s: [0; 12],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    UnInited,
    Ready,
    Running,
    Finished,
}

pub struct Task {
    pub context: usize,
    pub status: TaskStatus,
    pub task_id: Option<usize>,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            context: 0,
            status: TaskStatus::UnInited,
            task_id: None,
        }
    }
}

impl Task {
    pub fn new(addr: usize, id: usize) -> Self {
        //init trap context
        let trap_context = TrapContext::app_init_context(addr, USER_STACKS[id].stack_bottom());
        let task_context = TaskContext::app_init_task_context();
        let init_context = KERNEL_STACKS[id].prepare_runtime_stack(task_context, trap_context);
        Task {
            context: init_context,
            status: TaskStatus::Ready,
            task_id: Some(id),
        }
    }
    pub fn ptr_to_context(&self) -> *const usize {
        &self.context as *const usize
    }
    // pub fn mark_status(&mut self, next_status: TaskStatus) {
    //     self.status = next_status;
    // }
}

static USER_STACKS: [UserStack; MAX_JOB_NUM] = [UserStack {
    data: [0; APP_USER_STACK_SIZE],
}; MAX_JOB_NUM];

static KERNEL_STACKS: [KernelStack; MAX_JOB_NUM] = [KernelStack {
    data: [0; APP_KERNEL_STACK_SIZE],
}; MAX_JOB_NUM];

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; APP_KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; APP_USER_STACK_SIZE],
}

impl UserStack {
    fn stack_bottom(&self) -> usize {
        APP_USER_STACK_SIZE + self.data.as_ptr() as usize
    }
}

impl KernelStack {
    fn stack_bottom(&self) -> usize {
        APP_KERNEL_STACK_SIZE + self.data.as_ptr() as usize
    }

    fn prepare_runtime_stack(&self, task_context: TaskContext, trap_context: TrapContext) -> usize {
        let mut ptr = self.stack_bottom();
        ptr -= size_of::<TrapContext>();
        unsafe {
            *(ptr as *mut TrapContext) = trap_context;
        }
        ptr -= size_of::<TaskContext>();
        unsafe {
            *(ptr as *mut TaskContext) = task_context;
        }
        ptr
    }
}
