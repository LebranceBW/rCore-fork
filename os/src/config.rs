pub const MAX_JOB_NUM: usize = 6;
pub const KERNEL_HEAP_SIZE: usize = 0x50000;
// pub const APP_BASE_ADDRESS: usize = 0x80300000;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_ADDRESS_STEP: usize = 0x20000;
pub const APP_USER_STACK_SIZE: usize = 0x1000;
pub const APP_KERNEL_STACK_SIZE: usize = 0x20000;

pub const CLOCK_FREQ: usize = 12500000;
