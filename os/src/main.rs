#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]

extern crate alloc;
mod config;
mod console;
mod lang_items;
mod loader;
mod mm;
mod mylog;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;

use log::info;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    #[cfg(feature = "verbose")]
    mylog::init(log::LevelFilter::Trace);
    #[cfg(feature = "concise")]
    mylog::init(log::LevelFilter::Info);
    info!("Logger initialized.");
    print_section_info();
    mm::init_heap();
    info!("Heap initialized.");
    trap::init();
    info!("Trap initialized.");
    timer::init();
    info!("Timer initialized.");
    info!("First task start off.");
    task::start_running();
    unreachable!();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn print_section_info() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
}
