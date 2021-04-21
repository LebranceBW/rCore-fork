#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod config;
mod lang_items;
mod loader;
mod sbi;
mod std;
mod syscall;
mod task;
mod trap;
mod test;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    print_section_info();
    clear_bss();
    kernel_info!("Trap initialized.");
    trap::init();
    let (_apps, _) = loader::load_apps();
    // trap::run(apps[0]);
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
    log_info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    log_info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    log_info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    log_info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    log_info!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize,
        boot_stack_top as usize
    );
}
