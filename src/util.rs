use std::thread::sleep;
use std::time::{Duration, Instant};

use failure::{Error, Fail, Fallible};

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
    ///
    /// Note: If your predicate function shadows potential unexpected
    ///   errors you should consider using `#strict_until`.
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

    /// Wait until the given predicate returns `Ok(G)`, an unexpected error occurs or timeout arrives.
    ///
    /// Errors produced by the predicate are downcasted by the additional provided closure.
    /// If the downcast is successful - the error is ignored, otherwise the wait is terminated
    /// and `Err(error)` containing the unexpected failure is returned to the caller.
    ///
    /// You can use `failure::Error::downcast::<YourStructName>` out-of-the-box,
    /// if you need to ignore one expected error, or you can implement a matching closure
    /// that responds to multiple error types.
    pub fn strict_until<F, D, E, G>(&self, predicate: F, downcast: D) -> Fallible<G>
    where
        F: FnMut() -> Fallible<G>,
        D: FnMut(Error) -> Fallible<E>,
        E: Fail,
    {
        let mut predicate = predicate;
        let mut downcast = downcast;
        let start = Instant::now();
        loop {
            match predicate() {
                Ok(value) => return Ok(value),
                Err(error) => downcast(error)?,
            };

            if start.elapsed() > self.timeout {
                return Err(Timeout.into());
            }
            sleep(self.sleep);
        }
    }
}
