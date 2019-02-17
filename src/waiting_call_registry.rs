use failure::Error;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Mutex;

use crate::cdtp::{CallId, Response};
use crate::transport::ConnectionClosed;

trait IdentifiableResponse {
    fn call_id(&self) -> CallId;
}

pub struct WaitingCallRegistry {
    calls: Mutex<HashMap<CallId, mpsc::Sender<Result<Response, Error>>>>,
}

impl IdentifiableResponse for Response {
    fn call_id(&self) -> u16 {
        self.call_id
    }
}

impl Default for WaitingCallRegistry {
    fn default() -> Self {
        let calls = Mutex::new(HashMap::new());

        Self { calls }
    }
}

impl WaitingCallRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn resolve_call(&self, response: Response) {
        let waiting_call_tx: mpsc::Sender<Result<Response, Error>> = {
            let mut waiting_calls = self.calls.lock().unwrap();
            waiting_calls.remove(&response.call_id()).unwrap()
        };

        waiting_call_tx
            .send(Ok(response))
            .expect("failed to send response to waiting call");
    }

    pub fn register_call(&self, call_id: CallId) -> mpsc::Receiver<Result<Response, Error>> {
        let (tx, rx) = mpsc::channel::<Result<Response, Error>>();
        let mut calls = self.calls.lock().unwrap();
        calls.insert(call_id, tx);
        rx
    }

    pub fn unregister_call(&self, call_id: CallId) {
        let mut calls = self.calls.lock().unwrap();
        calls.remove(&call_id).unwrap();
    }

    pub fn cancel_outstanding_method_calls(&self) {
        let calls = self.calls.lock().unwrap();
        for (_call_id, sender) in calls.iter() {
            sender.send(Err(ConnectionClosed {}.into())).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn register_and_receive_calls() {
        env_logger::try_init().unwrap_or(());

        let waiting_calls = WaitingCallRegistry::new();

        let call_rx = waiting_calls.register_call(431);
        let resp = Response {
            call_id: 431,
            result: Some(json! {true}),
            error: None,
        };
        let resp_clone = resp.clone();

        let call_rx2 = waiting_calls.register_call(123);
        let resp2 = Response {
            call_id: 123,
            result: Some(json! {false}),
            error: None,
        };
        let resp2_clone = resp2.clone();

        waiting_calls.resolve_call(resp);
        waiting_calls.resolve_call(resp2);

        // note how they're in reverse order to that in which they were called!
        assert_eq!(resp2_clone, call_rx2.recv().unwrap().unwrap());
        assert_eq!(resp_clone, call_rx.recv().unwrap().unwrap());
    }
}
