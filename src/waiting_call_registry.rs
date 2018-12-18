use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;

type CallId = u16;

trait IdentifiableResponse {
    fn call_id(&self) -> CallId;
}

#[derive(Debug, PartialEq, Clone)]
struct CallResponse { call_id: CallId, data: bool }

struct WaitingCallRegistry {
    calls: Arc<Mutex<HashMap<CallId, mpsc::Sender<CallResponse>>>>
}


impl IdentifiableResponse for CallResponse {
    fn call_id(&self) -> u16 {
        self.call_id
    }
}

// TODO: find out how to make this generic!!!
impl WaitingCallRegistry
{
    pub fn new(incoming_responses: mpsc::Receiver<CallResponse>) -> Self
    {
        let calls = Arc::new(Mutex::new(HashMap::new()));

        let calls_mutex = Arc::clone(&calls);

        std::thread::spawn(move || {
            for response in incoming_responses.iter() {
                let mut waiting_calls = calls_mutex.lock().unwrap();

                let waiting_call_tx: mpsc::Sender<CallResponse>  = waiting_calls.remove(&response.call_id()).unwrap();

                waiting_call_tx.send(response);
            }
        });

        WaitingCallRegistry {
            calls
        }
    }

    fn handle_incoming(&mut self) {
    }

    pub fn register_call(&mut self, call_id: CallId) -> mpsc::Receiver<CallResponse> {
        let (tx, rx) = mpsc::channel::<CallResponse>();
        let calls_mutex = Arc::clone(&self.calls);
        let mut calls = calls_mutex.lock().unwrap();
        calls.insert(call_id, tx);
        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_receive_calls() {
        env_logger::try_init().unwrap_or(());

        let (responses_tx, responses_rx) = mpsc::channel::<CallResponse>();
        let mut waiting_calls = WaitingCallRegistry::new(responses_rx);

        // TODO: two at same time!

        let call_rx = waiting_calls.register_call(431);
        let resp = CallResponse { call_id: 431, data: true };

        let call_rx2 = waiting_calls.register_call(123);
        let resp2 = CallResponse { call_id: 123, data: true };

        responses_tx.send(resp.clone());
        responses_tx.send(resp2.clone());

        // note how they're in reverse order to that in which they were called!
        assert_eq!(resp2, call_rx2.recv().unwrap());
        assert_eq!(resp, call_rx.recv().unwrap());
    }
}
