use failure::{Error, Fail};
use std::time::{Duration, SystemTime};

#[derive(Debug, Fail)]
#[fail(display = "The thing you were waiting for never came")]
pub struct TimedOut {}

pub struct WaitOptions {
    pub timeout_ms: u64,
    pub sleep_ms: u64,
}

pub fn wait_for<F, G>(predicate: F, wait_options: WaitOptions) -> Result<G, Error>
where
    F: Fn() -> Option<G>,
{
    let time_before = SystemTime::now();
    loop {
        let elapsed = time_before.elapsed()?;

        if elapsed > Duration::from_millis(wait_options.timeout_ms) {
            return Err(TimedOut {}.into());
        }

        if let Some(thing) = predicate() {
            return Ok(thing);
        }

        std::thread::sleep(std::time::Duration::from_millis(wait_options.sleep_ms));
    }
}

pub fn wait_until_true<F>(predicate: F, wait_options: WaitOptions) -> Result<(), Error>
where
    F: Fn() -> bool,
{
    wait_for(
        || {
            if predicate() {
                Some(())
            } else {
                None
            }
        },
        wait_options,
    )
}

pub fn wait_for_mut<F, G>(mut predicate: F, wait_options: WaitOptions) -> Result<G, Error>
where
    F: FnMut() -> Option<G>,
{
    let time_before = std::time::SystemTime::now();
    loop {
        let elapsed = time_before.elapsed()?;

        if elapsed > Duration::from_millis(wait_options.timeout_ms) {
            return Err(TimedOut {}.into());
        }

        if let Some(thing) = predicate() {
            return Ok(thing);
        }

        std::thread::sleep(std::time::Duration::from_millis(wait_options.sleep_ms));
    }
}
