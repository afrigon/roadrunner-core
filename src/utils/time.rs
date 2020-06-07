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
        "{:02}h {:02}m {:02}s {:03}ms",
        elapsed.as_secs() / 3600,
        elapsed.as_secs() / 60 % 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis()
    )
}

/// formated time since program was started (to the millisecond)
pub fn time_since_launched() -> String {
    time_since(Instant::now() - *PROGRAM_START)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_since_format() {
        assert_eq!("00h 00m 00s 000ms", time_since(Duration::new(0, 0)));
        assert_eq!("00h 00m 00s 999ms", time_since(Duration::from_millis(999)));
        assert_eq!("00h 00m 01s 000ms", time_since(Duration::from_secs(1)));
        assert_eq!("00h 00m 59s 000ms", time_since(Duration::from_secs(59)));
        assert_eq!("00h 01m 00s 000ms", time_since(Duration::from_secs(60)));
        assert_eq!(
            "00h 59m 00s 000ms",
            time_since(Duration::from_secs(59 * 60))
        );
        assert_eq!(
            "01h 00m 00s 000ms",
            time_since(Duration::from_secs(60 * 60))
        );
        assert_eq!(
            "99h 00m 00s 000ms",
            time_since(Duration::from_secs(99 * 60 * 60))
        );
    }
}
