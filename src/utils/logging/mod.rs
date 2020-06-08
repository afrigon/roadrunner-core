mod file_logger;
mod multi_logger;
mod stdout_logger;

pub use file_logger::{FileLogger, FileLoggerOptions};
pub use log::{debug, error, info, trace, warn, Level, Log};
pub use stdout_logger::StdoutLogger;

pub fn init(loggers: Vec<Box<dyn Log>>) {
    let _ = log::set_boxed_logger(Box::new(multi_logger::MultiLogger { loggers }));
}
