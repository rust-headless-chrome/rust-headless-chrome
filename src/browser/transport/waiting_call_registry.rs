use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Mutex;

use failure::Fallible;
use log::*;

use crate::protocol::{CallId, Response};

use super::ConnectionClosed;

trait IdentifiableResponse {
    fn call_id(&self) -> CallId;
}

#[derive(Debug)]
pub struct WaitingCallRegistry {
    calls: Mutex<HashMap<CallId, mpsc::Sender<Fallible<Response>>>>,
}

impl IdentifiableResponse for Response {
    fn call_id(&self) -> CallId {
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

    pub fn resolve_call(&self, response: Response) -> Fallible<()> {
        trace!("Resolving call");
        let waiting_call_tx: mpsc::Sender<Fallible<Response>> = {
            let mut waiting_calls = self.calls.lock().unwrap();
            waiting_calls.remove(&response.call_id()).unwrap()
        };
        waiting_call_tx.send(Ok(response))?;
        Ok(())
    }

    pub fn register_call(&self, call_id: CallId) -> mpsc::Receiver<Fallible<Response>> {
        let (tx, rx) = mpsc::channel::<Fallible<Response>>();
        let mut calls = self.calls.lock().unwrap();
        calls.insert(call_id, tx);
        trace!("registered {:?}", call_id);
        rx
    }

    pub fn unregister_call(&self, call_id: CallId) {
        trace!("Deregistering call");
        let mut calls = self.calls.lock().unwrap();
        calls.remove(&call_id).unwrap();
    }

    // TODO: make it so we can pass in whatever error we want here
    // to make it less dependent on browser::transport
    pub fn cancel_outstanding_method_calls(&self) {
        trace!("Cancelling outstanding method calls");
        let calls = self.calls.lock().unwrap();
        for (call_id, sender) in calls.iter() {
            trace!(
                "Telling waiting method call {:?} that the connection closed",
                call_id
            );
            if let Err(e) = sender.send(Err(ConnectionClosed {}.into())) {
                trace!(
                    "Couldn't send ConnectionClosed to waiting method call: {:?} because {:?}",
                    call_id,
                    e
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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

        waiting_calls.resolve_call(resp).unwrap();
        waiting_calls.resolve_call(resp2).unwrap();

        // note how they're in reverse order to that in which they were called!
        assert_eq!(resp2_clone, call_rx2.recv().unwrap().unwrap());
        assert_eq!(resp_clone, call_rx.recv().unwrap().unwrap());
    }
}
