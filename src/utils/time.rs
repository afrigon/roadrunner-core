use std::time::Instant;
use std::time::SystemTime;

lazy_static! {
    pub static ref PROGRAM_START: Instant = Instant::now();
}

/// current amount of milliseconds elapsed since epoch
pub fn ms_since_epoch() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}
