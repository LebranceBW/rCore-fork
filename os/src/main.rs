#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]

mod lang_items;
mod sbi;
use crate::sbi::{console_putchar, shutdown};
use core::fmt::{self, Write};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    println!("Hello world");
    panic!("It should shutdown!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
        ($fmt: literal $(, $($arg: tt)+)?) => {
                    $crate::console::print(format_args!($fmt $(, $($arg)+)?));
                        }
}

#[macro_export]
macro_rules! println {
        ($fmt: literal $(, $($arg: tt)+)?) => {
                    print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
                        }
}
