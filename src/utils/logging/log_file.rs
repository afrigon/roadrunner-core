use log::{Level, Log, Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

pub struct LogFile {
    file: Mutex<File>,
    level: Level,
}

impl LogFile {
    pub fn new<P: AsRef<Path>>(path: P, level: Level) -> Self {
        Self {
            file: Mutex::new(File::create(path).unwrap()),
            level,
        }
    }
}

impl Log for LogFile {
    fn enabled(&self, record: &Metadata<'_>) -> bool {
        record.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        let mut lock = self.file.lock().unwrap();
        let line = format!("{} - {}", record.level(), record.args());
        lock.write(line.as_bytes()).unwrap();
    }

    fn flush(&self) {
        let mut lock = self.file.lock().unwrap();
        lock.flush().unwrap();
    }
}
