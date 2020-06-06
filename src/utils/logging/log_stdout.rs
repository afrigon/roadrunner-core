use log::{Level, Log, Metadata, Record};
use std::io::{stdout, Stdout, Write};
use std::sync::Mutex;

pub struct LogStdOut {
    stdout: Mutex<Stdout>,
    level: Level,
}

impl LogStdOut {
    pub fn new(level: Level) -> Self {
        Self {
            stdout: Mutex::new(stdout()),
            level,
        }
    }
}

impl Log for LogStdOut {
    fn enabled(&self, record: &Metadata<'_>) -> bool {
        record.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        let mut lock = self.stdout.lock().unwrap();
        let line = format!("{} - {}", record.level(), record.args());
        lock.write(line.as_bytes()).unwrap();
    }

    fn flush(&self) {
        let mut lock = self.stdout.lock().unwrap();
        lock.flush().unwrap();
    }
}
