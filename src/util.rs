use std::time::{Duration, Instant};
use std::{collections::HashMap, thread::sleep};

use anyhow::{Error, Result};

use thiserror::Error;

use crate::protocol::cdp::Runtime::RemoteObject;

use crate::browser::tab::point::Point;

#[derive(Debug, Error)]
#[error("The event waited for never came")]
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

pub fn extract_midpoint(remote_obj: RemoteObject) -> Result<Point> {
    let mut prop_map = HashMap::new();

    match remote_obj.preview.map(|v| {
        for prop in v.properties {
            prop_map.insert(prop.name, prop.value.unwrap().parse::<f64>().unwrap());
        }
        Point {
            x: prop_map["x"] + (prop_map["width"] / 2.0),
            y: prop_map["y"] + (prop_map["height"] / 2.0),
        }
    }) {
        Some(v) => Ok(v),
        None => Ok(Point { x: 0.0, y: 0.0 }),
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
    pub fn strict_until<F, D, E, G>(&self, predicate: F, downcast: D) -> Result<G>
    where
        F: FnMut() -> Result<G>,
        D: FnMut(Error) -> Result<E>,
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
