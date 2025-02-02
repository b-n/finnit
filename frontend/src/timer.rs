use std::time::{Duration, SystemTime, SystemTimeError};

pub struct Timer {
    last: SystemTime,
    every: Option<Duration>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            last: SystemTime::now(),
            every: None,
        }
    }

    pub fn every(mut self, duration: Duration) -> Self {
        self.every = Some(duration);
        self
    }

    pub fn check_every(&mut self) -> Result<bool, SystemTimeError> {
        let elapsed = self.last.elapsed()?;

        // Short circuit None case to never succeed
        let every = self.every.unwrap_or(Duration::MAX);

        if elapsed < every {
            return Ok(false);
        }

        self.last = self
            .last
            .checked_add(every)
            .expect("Time overflow, `every` was likely too large");

        Ok(true)
    }
}
