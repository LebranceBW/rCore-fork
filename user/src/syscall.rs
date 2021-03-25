const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut res: isize;
    unsafe {
        llvm_asm!("ecall"
                  : "={x10}" (res)
                  : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
                  : "memory"
                  : "volatile");
    }
    res
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
