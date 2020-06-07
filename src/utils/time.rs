use std::time::Duration;
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

pub fn time_since(elapsed: Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        elapsed.as_secs() / 3600,
        elapsed.as_secs() / 60 % 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis()
    )
}

/// formated time since program was started (to the millisecond)
pub fn time_since_launched() -> String {
    time_since(PROGRAM_START.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_since_format() {
        assert_eq!("00:00:00.000", time_since(Duration::new(0, 0)));
        assert_eq!("00:00:00.999", time_since(Duration::from_millis(999)));
        assert_eq!("00:00:01.000", time_since(Duration::from_secs(1)));
        assert_eq!("00:00:59.000", time_since(Duration::from_secs(59)));
        assert_eq!("00:01:00.000", time_since(Duration::from_secs(60)));
        assert_eq!("00:59:00.000", time_since(Duration::from_secs(59 * 60)));
        assert_eq!("01:00:00.000", time_since(Duration::from_secs(60 * 60)));
        assert_eq!(
            "99:00:00.000",
            time_since(Duration::from_secs(99 * 60 * 60))
        );
    }
}
