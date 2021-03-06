use crate::config::*;
use crate::trap::TrapContext;
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

pub const TIME_SLICE: usize = 10;
const BIG_STRIDE: usize = 699999999;

#[derive(Eq, Ord)]
pub struct TaskControlBlock {
    pub context: usize,
    pub priority: usize,
    pub stride: usize,
    pub task_id: usize,
    pub total_time_ms: usize,
}

impl core::cmp::PartialEq for TaskControlBlock {
    fn eq(&self, other: &Self) -> bool {
        self.stride.eq(&other.stride)
    }
}

impl core::cmp::PartialOrd for TaskControlBlock {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.stride
            .partial_cmp(&other.stride)
            .map(core::cmp::Ordering::reverse)
    }
}

impl TaskControlBlock {
    pub fn new(addr: usize, id: usize) -> Self {
        //init trap context
        let trap_context = TrapContext::app_init_context(addr, USER_STACKS[id].stack_bottom());
        let task_context = TaskContext::app_init_task_context();
        let init_context = KERNEL_STACKS[id].prepare_runtime_stack(task_context, trap_context);
        Self {
            context: init_context,
            task_id: id,
            priority: 16,
            stride: 0,
            total_time_ms: 0,
        }
    }
    pub fn ptr_to_context(&self) -> *const usize {
        &self.context as *const usize
    }
    pub fn set_priority(&mut self, prio: isize) -> isize {
        if prio >= 2 {
            self.priority = prio as usize;
            prio
        } else {
            -1
        }
    }
    pub fn update_stride(&mut self) {
        self.total_time_ms += TIME_SLICE;
        self.stride += BIG_STRIDE / self.priority
    }
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
