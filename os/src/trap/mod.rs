use crate::kernel_info;
use crate::syscall::*;
use riscv::register::mtvec::TrapMode;
use riscv::register::scause::{Exception, Trap};
use riscv::register::sstatus::Sstatus;
use riscv::register::sstatus::SPP;
use riscv::register::{scause, sstatus, stval, stvec};
use crate::config::*;

global_asm!(include_str!("trap.S"));
extern "C" {
    fn __trap_entrance();
    // fn __restore(cx_addr: usize); 
}

pub fn init() {
    unsafe {
        stvec::write(__trap_entrance as usize, TrapMode::Direct);
    }
}

// #[repr(align(4096))]
// struct KernelStack {
//     data: [u8; APP_KERNEL_STACK_SIZE],
// }

// #[repr(align(4096))]
// struct UserStack {
//     data: [u8; APP_USER_STACK_SIZE],
// }

// static KERNEL_STACK: KernelStack = KernelStack { data: [0; APP_KERNEL_STACK_SIZE] };
// static USER_STACK: UserStack = UserStack { data: [0; APP_USER_STACK_SIZE] };


// impl KernelStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + APP_KERNEL_STACK_SIZE
//     }
//     pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
//         let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
//         unsafe { *cx_ptr = cx; }
//         unsafe { cx_ptr.as_mut().unwrap() }
//     }
// }

// impl UserStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + APP_USER_STACK_SIZE
//     }

// }

// pub fn run(addr:usize) {
//     unsafe {
//         __restore(KERNEL_STACK.push_context(
//             TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.get_sp())
//         ) as *const _ as usize);
//     }
// }

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            panic!("PageFault in application, core dumped.");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            panic!("IllegalInstruction in application, core dumped.");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}, scause = {:#x}!",
                scause.cause(),
                stval,
                scause.bits()
            );
        }
    }
    cx
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapContext {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
