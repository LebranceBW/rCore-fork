const SYSCALL_WRITE: usize = 64;
const SYSCALL_YIELD: usize = 65;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_SET_PRIORITY: usize = 140;

mod fs;
mod process;

use fs::*;
use process::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_SET_PRIORITY => sys_set_priority(args[0]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
