use failure::Fail;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug, Fail)]
#[fail(display = "The event waited for never came")]
pub struct Timeout;

/// A helper to wait until some event has passed.
#[derive(Debug)]
pub struct Wait {
    timeout: Duration,
    sleep: Duration,
}

impl Default for Wait {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            sleep: Duration::from_millis(100),
        }
    }
}

impl Wait {
    pub fn new(timeout: Duration, sleep: Duration) -> Self {
        Self { timeout, sleep }
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            timeout,
            ..Self::default()
        }
    }

    pub fn with_sleep(sleep: Duration) -> Self {
        Self {
            sleep,
            ..Self::default()
        }
    }

    pub fn forever() -> Self {
        Self {
            timeout: Duration::from_secs(u64::max_value()),
            ..Self::default()
        }
    }

    /// Wait until the given predicate returns `Some(G)` or timeout arrives.
    pub fn until<F, G>(&self, predicate: F) -> Result<G, Timeout>
    where
        F: FnMut() -> Option<G>,
    {
        let mut predicate = predicate;
        let start = Instant::now();
        loop {
            if let Some(v) = predicate() {
                return Ok(v);
            }
            if start.elapsed() > self.timeout {
                return Err(Timeout);
            }
            sleep(self.sleep);
        }
    }
}
