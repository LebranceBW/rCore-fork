use crate::sbi::console_putchar;
use core::fmt::{self, Write};

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
        $crate::std::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::std::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! log{
    ($color: literal, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::std::print(format_args!(concat!($color, $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! log_error{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!("\x1b[31m", $fmt $(, $($arg)+)?)
    }
}

#[macro_export]
macro_rules! log_warn{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!("\x1b[93m", $fmt $(, $($arg)+)?)
    }
}

#[macro_export]
macro_rules! log_info{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!("\x1b[34m", $fmt $(, $($arg)+)?)
    }
}

#[macro_export]
macro_rules! log_debug{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!("\x1b[32m", $fmt $(, $($arg)+)?)
    }
}

#[macro_export]
macro_rules! log_trace{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!("\x1b[90m", $fmt $(, $($arg)+)?)
    }
}
