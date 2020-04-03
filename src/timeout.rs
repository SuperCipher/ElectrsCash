use crate::errors::*;
use std::time::{Duration, Instant};

struct TimeoutTrigger {
    start: Instant,
    timeout: Duration,
}

impl TimeoutTrigger {
    pub fn new(timeout: Duration) -> TimeoutTrigger {
        TimeoutTrigger {
            start: Instant::now(),
            timeout: timeout,
        }
    }

    pub fn check(&self) -> Result<()> {
        if self.start.elapsed() >= self.timeout {
            return Err("Timeout".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_timeout() {
        let timeout = TimeoutTrigger::new(Duration::from_millis(50));
        assert!(!timeout.check().is_err());
        sleep(Duration::from_millis(50));
        assert!(timeout.check().is_err());
    }
}
