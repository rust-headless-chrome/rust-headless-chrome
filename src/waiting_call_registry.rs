use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

use crate::cdtp::{CallId, Response};

trait IdentifiableResponse {
    fn call_id(&self) -> CallId;
}


pub struct WaitingCallRegistry {
    calls: Arc<Mutex<HashMap<CallId, mpsc::Sender<Response>>>>
}


impl IdentifiableResponse for Response {
    fn call_id(&self) -> u16 {
        self.call_id
    }
}

// TODO: find out how to make this generic!!!
impl WaitingCallRegistry
{
    pub fn new(incoming_responses: mpsc::Receiver<Response>) -> Self
    {
        let calls = Arc::new(Mutex::new(HashMap::new()));

        let calls_mutex = Arc::clone(&calls);

        std::thread::spawn(move || {
            for response in incoming_responses.iter() {
                let mut waiting_calls = calls_mutex.lock().unwrap();

                let waiting_call_tx: mpsc::Sender<Response>  = waiting_calls.remove(&response.call_id()).unwrap();

                waiting_call_tx.send(response).expect("failed to send response to waiting call");
            }
        });

        WaitingCallRegistry {
            calls
        }
    }

    pub fn register_call(&self, call_id: CallId) -> mpsc::Receiver<Response> {
        let (tx, rx) = mpsc::channel::<Response>();
        let calls_mutex = Arc::clone(&self.calls);
        let mut calls = calls_mutex.lock().unwrap();
        calls.insert(call_id, tx);
        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn register_and_receive_calls() {
        env_logger::try_init().unwrap_or(());

        let (responses_tx, responses_rx) = mpsc::channel::<Response>();
        let waiting_calls = WaitingCallRegistry::new(responses_rx);

        let call_rx = waiting_calls.register_call(431);
        let resp = Response { call_id: 431, result: json!{true} };

        let call_rx2 = waiting_calls.register_call(123);
        let resp2 = Response { call_id: 123, result: json!{false} };

        responses_tx.send(resp.clone()).unwrap();
        responses_tx.send(resp2.clone()).unwrap();

        // note how they're in reverse order to that in which they were called!
        assert_eq!(resp2, call_rx2.recv().unwrap());
        assert_eq!(resp, call_rx.recv().unwrap());
    }
}
