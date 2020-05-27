use log::LevelFilter;
use log::{Level, Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

struct Logger {
    file: Mutex<File>,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut lock = self.file.lock().unwrap();
            writeln!(lock, "{} - {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_boxed_logger(Box::new(Logger {
        file: Mutex::new(File::create("logs").unwrap()),
    }))
    .unwrap();
    log::set_max_level(LevelFilter::Info);
}
