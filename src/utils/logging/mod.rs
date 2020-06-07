pub mod log_file;
pub mod log_stdout;
pub mod multi_logger;

pub use log::{debug, error, info, trace, warn, Level, Log};
pub use log_file::LogFile;
pub use log_stdout::LogStdOut;

pub fn init(loggers: Vec<Box<dyn Log>>) {
    let _ = log::set_boxed_logger(Box::new(multi_logger::MultiLogger { loggers }));
    log::set_max_level(log::LevelFilter::Info);
}
