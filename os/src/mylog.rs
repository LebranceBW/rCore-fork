use crate::println;
use log::{Level, LevelFilter, Metadata, Record};

const COLOR_FMT_RED: &str = "\x1b[31m";
const COLOR_FMT_YELLOW: &str = "\x1b[93m";
const COLOR_FMT_WHITE: &str = "\x1b[m";
const COLOR_FMT_GRAY: &str = "\x1b[90m";
const COLOR_FMT_BLUE: &str = "\x1b[34m";
const FMT_END: &str = "\x1b[0m";

pub struct SimpleLogger;

pub static logger: SimpleLogger = SimpleLogger {};

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Trace
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let ctl_color = match record.level() {
                Level::Info => COLOR_FMT_WHITE,
                Level::Debug => COLOR_FMT_BLUE,
                Level::Error => COLOR_FMT_RED,
                Level::Trace => COLOR_FMT_GRAY,
                Level::Warn => COLOR_FMT_YELLOW,
            };
            println!(
                "{}[kernel {}] {}{}",
                ctl_color,
                record.level(),
                record.args(),
                FMT_END
            );
        }
    }

    fn flush(&self) {}
}

pub fn init(level: LevelFilter) {
    log::set_logger(&logger).unwrap();
    log::set_max_level(level);
}
