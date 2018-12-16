use std::io::Read;
use std::process::{Command, Stdio, Child};
use std::fmt;
use std::borrow::BorrowMut;

use log::*;

use error_chain::bail;

use regex::Regex;

use super::errors::*;

#[derive(Debug)]
pub struct BrowserId(String);

impl fmt::Display for BrowserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Chrome {
    child_process: Child,
    pub browser_id: BrowserId,
}

impl Chrome {
    // TODO: find out why this complains if named 'new'
    pub fn new(headless: bool) -> Result<Self> {
        info!("Trying to start Chrome");

//        let process = Command::new("/usr/bin/google-chrome")
        let mut args = vec![// "--headless",
                            "--remote-debugging-port=9393", "--verbose"];

        if headless {
            args.extend(&["--headless"]);
        }

        let mut process = Command::new("/home/alistair/Downloads/chrome-linux/chrome")
            .args(&args)
            .stderr(Stdio::piped())
            .spawn()
            .chain_err(|| "Couldn't start chrome")?;

        info!("Started Chrome. PID: {}", process.id());

        let browser_id = Chrome::browser_id_from_output(process.borrow_mut())
            .chain_err(|| "Couldn't get browser ID from Chrome process")?;

        Ok(Chrome {
            child_process: process,
            browser_id: browser_id,
        })
    }

    fn browser_id_from_output(child_process: &mut Child) -> Result<BrowserId> {
        // TODO: user static or lazy static regex
        let re = Regex::new(r"listening on .*/devtools/browser/(.*)\n").unwrap();

        let extract = |text: &str| -> Option<String> {
            let caps = re.captures(text);
            let cap = &caps?[1];
            Some(cap.into())
        };

        let mut buf = [0; 512];

        let time_before = std::time::SystemTime::now();
        loop {
            let elapsed_seconds = time_before
                .elapsed()
                .chain_err(|| "Couldn't get system time")?
                .as_secs();

            if elapsed_seconds > 1 {
                bail!("Couldn't read WebSocket URL within one second");
            }

            let my_stderr = child_process.stderr.as_mut();
            let bytes_read = my_stderr.unwrap().read(&mut buf).unwrap();

            if bytes_read > 0 {
                let chrome_output = String::from_utf8_lossy(&buf);
                debug!("Chrome output: {}", chrome_output);

                match extract(&chrome_output) {
                    Some(browser_id) => return Ok(BrowserId(browser_id)),
                    None => continue
                };
            }
        }
    }
}

impl Drop for Chrome {
    fn drop(&mut self) {
        debug!("killing chrome PID: {}", self.child_process.id());
        self.child_process.kill().unwrap();
        self.child_process.wait().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    fn current_child_pids() -> Vec<i32> {
        let current_pid = std::process::id();
        let mut current_process_children_file = File::open(format!("/proc/{}/task/{}/children", current_pid, current_pid)).unwrap();
        let mut child_pids = String::new();
        current_process_children_file.read_to_string(&mut child_pids).unwrap();
        return child_pids.split_whitespace().map(|pid_str| pid_str.parse::<i32>().unwrap() ).collect();
    }

    #[test]
    fn kills_process_on_drop() {
        env_logger::try_init().unwrap_or(());
        let time_before = std::time::SystemTime::now();
        {
            let _chrome = &mut super::Chrome::new(true).unwrap();

            let chrome_startup_millis = time_before
                .elapsed()
                .unwrap()
                .as_millis();
            dbg!(chrome_startup_millis);
        }

        let child_pids = current_child_pids();
        assert!(child_pids.is_empty());
    }
}
