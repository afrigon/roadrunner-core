use log::{Level, Log, Metadata, Record};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::{io, io::Write};

use crate::utils::time::{ms_since_epoch, time_since_launched};

const LOG_DIR: &str = "logs/";
const CURRENT_LOG: &str = "current.log";
const MAX_LOG_SIZE: u64 = 5 * 1024 * 1024; // 5MB
const LOGS_TO_KEEP: usize = 10;

pub struct LogFile {
    level: Level,
}

impl LogFile {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

/// path to the current log file
fn log_path() -> PathBuf {
    Path::new(LOG_DIR).join(CURRENT_LOG)
}

/// handle to the current log file, creating it if necessary
fn log_file() -> io::Result<fs::File> {
    let create_file = |_| {
        let _ = fs::create_dir_all(Path::new(LOG_DIR));
        fs::File::create(log_path())
    };
    fs::OpenOptions::new()
        .append(true)
        .open(log_path())
        .or_else(create_file)
}

/// save current log but only keep n most recent files
fn rotate_logs() {
    // backup current log by timestamping it
    let new_file_path = Path::new(LOG_DIR).join(format!("{}.log", ms_since_epoch()));
    if let Err(err) = fs::rename(log_path(), new_file_path) {
        println!("ERROR: could not rotate log files - {}", err);
    }

    // remove oldest log file
    let count = fs::read_dir(LOG_DIR).map(|dir| dir.count()).ok();
    if count > Some(LOGS_TO_KEEP) {
        let _ = fs::read_dir(LOG_DIR).map(|dir_entry: fs::ReadDir| {
            // remove first file in lexicographical order (oldest for timestamped files)
            dir_entry
                .filter_map(|entry| entry.ok())
                .flat_map(|entry: fs::DirEntry| {
                    entry
                        .path()
                        .file_stem()
                        .and_then(|s| s.to_str().map(|s| s.to_string()))
                }) // iterator over file stems, as String (representing timestamps)
                .min() // gets the first, so the oldest
                .and_then(|stem: String| {
                    fs::remove_file(Path::new(LOG_DIR).join(format!("{}.log", stem))).ok()
                })
        });
    }
}

/// rotating file implementation for log
impl Log for LogFile {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        meta.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if let Ok(file) = log_file().as_mut() {
            let data = format!(
                "{} {}: {}\n",
                time_since_launched(),
                record.level(),
                record.args()
            );

            let _ = file.write(data.as_bytes());

            if let Ok(size) = file.metadata().map(|meta| meta.len()) {
                if size >= MAX_LOG_SIZE {
                    rotate_logs();
                }
            }
        } else {
            println!("ERROR: could not open log file");
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_enabled_debug() {
        let logger = LogFile::new(log::Level::Debug);
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }

    #[test]
    fn log_enabled_error() {
        let logger = LogFile::new(log::Level::Error);
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }
}
