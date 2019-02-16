use std::borrow::BorrowMut;
use std::io::{BufRead, BufReader};
use std::net;
use std::process::{Child, Command, Stdio};

use failure::{Error, Fail};
use log::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;

use crate::helpers::{wait_for_mut, WaitOptions};

//use crate::page_session::PageSession;
//use crate::tab::Tab;

pub struct Process {
    _child_process: TemporaryProcess,
    pub debug_ws_url: String,
}

#[derive(Debug, Fail)]
enum ChromeLaunchError {
    #[fail(display = "Chrome launched, but didn't give us a WebSocket URL before we timed out")]
    PortOpenTimeout,
    #[fail(display = "There are no available ports between 8000 and 9000 for debugging")]
    NoAvailablePorts,
    #[fail(display = "The chosen debugging port is already in use")]
    DebugPortInUse,
}

struct TemporaryProcess(Child);

impl Drop for TemporaryProcess {
    fn drop(&mut self) {
        info!("Killing Chrome. PID: {}", self.0.id());
        self.0.kill().unwrap();
        self.0.wait().unwrap();
    }
}

pub struct LaunchOptions<'a> {
    pub headless: bool,
    pub port: Option<u16>,
    pub path: &'a str,
}

impl<'a> Default for LaunchOptions<'a> {
    fn default() -> Self {
        LaunchOptions {
            headless: true,
            port: None,
            path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        }
    }
}

impl Process {
    pub fn new(launch_options: LaunchOptions) -> Result<Self, Error> {
        info!("Trying to start Chrome");

        let mut process = Process::start_process(&launch_options)?;

        info!("Started Chrome. PID: {}", process.0.id());

        let url;
        let mut attempts = 0;
        loop {
            if attempts > 50 {
                return Err(ChromeLaunchError::NoAvailablePorts {}.into());
            }

            match Process::ws_url_from_output(process.0.borrow_mut()) {
                Ok(debug_ws_url) => {
                    url = debug_ws_url;
                    break;
                }
                Err(error) => {
                    if launch_options.port.is_none() {
                        process = Process::start_process(&launch_options)?;
                    } else {
                        return Err(error);
                    }
                }
            }

            trace!(
                "Trying again to find available debugging port. Attempts: {}",
                attempts
            );
            attempts += 1;
        }

        Ok(Process {
            _child_process: process,
            debug_ws_url: url,
        })
    }

    fn start_process(launch_options: &LaunchOptions) -> Result<TemporaryProcess, Error> {
        let debug_port = if let Some(port) = launch_options.port {
            port
        } else {
            get_available_port().ok_or(ChromeLaunchError::NoAvailablePorts {})?
        };
        let port_option = format!("--remote-debugging-port={}", debug_port);

        // NOTE: picking random data dir so that each a new browser instance is launched
        // (see man google-chrome)
        let user_data_dir = ::tempfile::Builder::new()
            .prefix("rust-headless-chrome-profile")
            .tempdir()?;
        let data_dir_option = format!("--user-data-dir={}", user_data_dir.path().to_str().unwrap());

        trace!("Chrome will have profile: {}", data_dir_option);

        let mut args = vec![
            port_option.as_str(),
            "--verbose",
            "--no-first-run",
            data_dir_option.as_str(),
            //            "--window-size=1920,1080"
        ];

        if launch_options.headless {
            args.extend(&["--headless"]);
        }

        let process = TemporaryProcess(
            Command::new(launch_options.path)
                .args(&args)
                .stderr(Stdio::piped())
                .spawn()?,
        );
        Ok(process)
    }

    fn ws_url_from_output(child_process: &mut Child) -> Result<String, Error> {
        let port_taken = "Address already in use";

        let re = Regex::new(r"listening on (.*/devtools/browser/.*)$").unwrap();

        let extract = |text: &str| -> Option<String> {
            let caps = re.captures(text);
            let cap = &caps?[1];
            Some(cap.into())
        };

        let chrome_output_result = wait_for_mut(
            || {
                let my_stderr = BufReader::new(child_process.stderr.as_mut().unwrap());
                for line in my_stderr.lines() {
                    let chrome_output = line.ok()?;
                    trace!("Chrome output: {}", chrome_output);

                    if chrome_output.contains(port_taken) {
                        return None;
                    }

                    let answer = extract(&chrome_output);
                    if answer.is_some() {
                        return answer;
                    }
                }

                None
            },
            WaitOptions {
                timeout_ms: 200,
                sleep_ms: 10,
            },
        );

        if let Ok(output) = chrome_output_result {
            if output.contains(port_taken) {
                Err(ChromeLaunchError::DebugPortInUse {}.into())
            } else {
                Ok(output)
            }
        } else {
            Err(ChromeLaunchError::PortOpenTimeout {}.into())
        }
    }
}

fn get_available_port() -> Option<u16> {
    let mut ports: Vec<u16> = (8000..9000).collect();
    ports.shuffle(&mut thread_rng());
    ports.iter().find(|port| port_is_available(**port)).cloned()
}

fn port_is_available(port: u16) -> bool {
    net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use std::thread;

    fn current_child_pids() -> Vec<i32> {
        let current_pid = std::process::id();
        let mut current_process_children_file = File::open(format!(
            "/proc/{}/task/{}/children",
            current_pid, current_pid
        ))
        .unwrap();
        let mut child_pids = String::new();
        current_process_children_file
            .read_to_string(&mut child_pids)
            .unwrap();
        child_pids
            .split_whitespace()
            .map(|pid_str| pid_str.parse::<i32>().unwrap())
            .collect()
    }

    #[test]
    fn kills_process_on_drop() {
        env_logger::try_init().unwrap_or(());
        {
            let _chrome = &mut super::Process::new(Default::default()).unwrap();
        }

        let child_pids = current_child_pids();
        assert!(child_pids.is_empty());
    }

    #[test]
    fn launch_multiple_non_headless_instances() {
        env_logger::try_init().unwrap_or(());

        let mut handles = Vec::new();

        for _ in 0..10 {
            let handle = thread::spawn(|| {
                // these sleeps are to make it more likely the chrome startups will overlap
                std::thread::sleep(std::time::Duration::from_millis(10));
                let chrome = super::Process::new(super::LaunchOptions {
                    port: None,
                    ..Default::default()
                })
                .unwrap();
                std::thread::sleep(std::time::Duration::from_millis(100));
                chrome.debug_ws_url.clone()
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn no_instance_sharing() {
        env_logger::try_init().unwrap_or(());

        let mut handles = Vec::new();

        for _ in 0..10 {
            let chrome = super::Process::new(super::LaunchOptions {
                headless: false,
                ..Default::default()
            })
            .unwrap();
            handles.push(chrome);
        }
    }
}
