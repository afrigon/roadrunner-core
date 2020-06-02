use log::{LevelFilter, Metadata, Record};
use std::fs::File;
use std::io::{Stdout, Write};
use std::path::Path;
use std::sync::Mutex;

pub use log::{debug, error, info, warn, Level, Log};

pub struct Logger<W: Write + Send + Sync> {
    output: Mutex<W>,
    level: Level,
}

impl<W: Write + Sync + Send> log::Log for Logger<W> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut lock = self.output.lock().unwrap();
            writeln!(lock, "{} - {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}

impl<W: Write + Sync + Send> Logger<W> {
    pub fn new(output: W, level: Level) -> Self {
        Self {
            output: Mutex::new(output),
            level,
        }
    }
}

impl Logger<File> {
    pub fn to_file<P: AsRef<Path>>(path: P, level: Level) -> Self {
        Logger::new(File::create(path).unwrap(), level)
    }
}

impl Logger<Stdout> {
    pub fn to_stdout(level: Level) -> Self {
        Logger::new(std::io::stdout(), level)
    }
}

struct MultiLogger {
    loggers: Vec<Box<dyn log::Log>>,
}

impl log::Log for MultiLogger {
    fn enabled(&self, record: &log::Metadata<'_>) -> bool {
        self.loggers.iter().any(|logger| logger.enabled(record))
    }

    fn log(&self, record: &log::Record<'_>) {
        self.loggers
            .iter()
            .filter(|logger| logger.enabled(record.metadata()))
            .for_each(|logger| logger.log(record));
    }

    fn flush(&self) {
        self.loggers.iter().for_each(|logger| logger.flush());
    }
}

pub fn init(loggers: Vec<Box<dyn log::Log>>) {
    log::set_boxed_logger(Box::new(MultiLogger { loggers: loggers })).unwrap();
    log::set_max_level(LevelFilter::Info);
}
