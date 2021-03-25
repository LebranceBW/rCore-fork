use crate::kernel_info;
use crate::trap::TrapContext;
use core::cell::RefCell;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use lazy_static::*;
use crate::config::*;

struct AppManager {
    inner: RefCell<AppManagerInner>,
}

unsafe impl Sync for AppManager {}

struct AppManagerInner {
    current_job: usize,
    jobs: [usize; MAX_JOB_NUM + 1],
    num_jobs: usize,
}

impl AppManagerInner {
    pub fn print_app_info(&self) {
        kernel_info!("num_app = {}", self.num_jobs);
        for i in 0..self.num_jobs {
            kernel_info!("app_{} [{:#x}, {:#x})", i, self.jobs[i], self.jobs[i + 1]);
        }
    }

    unsafe fn load_app(&self, job_id: usize) {
        if job_id >= self.num_jobs {
            panic!("All applications completed!");
        }
        //clear icache
        llvm_asm!("fence.i" :::: "volatile");
        (APP_BASE_ADDRESS..APP_BASE_ADDRESS + APP_SIZE_LIMIT).for_each(|addr| {
            (addr as *mut u8).write_volatile(0);
        });
        let app_src = from_raw_parts(
            self.jobs[job_id] as *const u8,
            self.jobs[job_id + 1] - self.jobs[job_id],
        );
        let app_dst = from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

    pub fn get_current(&self) -> usize {
        self.current_job
    }

    pub fn move_to_next(&mut self) {
        self.current_job += 1;
    }
}

lazy_static! {
    static ref APP_MANAGER: AppManager = AppManager {
        inner: RefCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_job_ptr = _num_app as usize as *const usize;
            let num_jobs = unsafe { num_job_ptr.read_volatile() };
            let mut jobs: [usize; MAX_JOB_NUM + 1] = [0; MAX_JOB_NUM + 1];
            let jobs_raw: &[usize] = unsafe { from_raw_parts(num_job_ptr.add(1), num_jobs + 1) };
            jobs[..=num_jobs].copy_from_slice(jobs_raw);
            AppManagerInner {
                num_jobs,
                current_job: 0,
                jobs,
            }
        }),
    };
}

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

pub fn run_next_app() -> ! {
    let current_app = APP_MANAGER.inner.borrow().get_current();
    unsafe {
        APP_MANAGER.inner.borrow().load_app(current_app);
    }
    APP_MANAGER.inner.borrow_mut().move_to_next();
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}

pub fn init() {
    APP_MANAGER.inner.borrow().print_app_info();
}
