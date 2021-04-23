#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod config;
mod console;
mod lang_items;
mod loader;
mod mylog;
mod sbi;
mod syscall;
mod task;
mod test;
mod timer;
mod trap;

use log::info;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    print_section_info();
    clear_bss();
    mylog::init(log::LevelFilter::Info);
    info!("Logger initialized.");
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
    clear_bss();
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
}
