use log::LevelFilter;
use log::{Level, Metadata, Record};
use std::fs::File;
use std::io::Stdout;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

pub use log::{debug, info, warn, Log};

pub struct Logger<W: Write + Send + Sync> {
    output: Mutex<W>,
}

impl<W: Write + Sync + Send> log::Log for Logger<W> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
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
    pub fn new(output: W) -> Self {
        Self {
            output: Mutex::new(output),
        }
    }
}

impl Logger<File> {
    pub fn to_file<P: AsRef<Path>>(path: P) -> Self {
        Logger::new(File::create(path).unwrap())
    }
}

impl Logger<Stdout> {
    pub fn to_stdout() -> Self {
        Logger::new(std::io::stdout())
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
        for logger in self.loggers.iter() {
            logger.log(record);
        }
    }

    fn flush(&self) {
        for logger in self.loggers.iter() {
            logger.flush();
        }
    }
}

pub fn init(loggers: Vec<Box<dyn log::Log>>) {
    log::set_boxed_logger(Box::new(MultiLogger { loggers: loggers })).unwrap();
    log::set_max_level(LevelFilter::Info);
}
