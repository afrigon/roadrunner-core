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

/// formated time since program was started (to the millisecond)
pub fn time_since_launched() -> String {
    let elapsed = PROGRAM_START.elapsed();
    format!(
        "{:02}h {:02}m {:02}s {:03}ms",
        elapsed.as_secs() / 3600,
        elapsed.as_secs() / 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis()
    )
}
