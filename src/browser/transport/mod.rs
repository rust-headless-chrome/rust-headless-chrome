use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use failure::{Error, Fail};
use log::*;
use serde;

use crate::cdtp;
use crate::cdtp::target;
use crate::cdtp::Event;
use crate::cdtp::Message;

use crate::cdtp::{CallId, MethodCall};
use waiting_call_registry::WaitingCallRegistry;
use web_socket_connection::WebSocketConnection;

mod waiting_call_registry;
mod web_socket_connection;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SessionId {
    fn from(session_id: String) -> Self {
        Self(session_id)
    }
}

#[derive(Eq, PartialEq, Hash)]
enum ListenerId {
    SessionId(SessionId),
    Browser,
}

type Listeners = Arc<Mutex<HashMap<ListenerId, Sender<Event>>>>;

pub struct Transport {
    web_socket_connection: WebSocketConnection,
    waiting_call_registry: Arc<WaitingCallRegistry>,
    listeners: Listeners,
    open: Arc<AtomicBool>,
    call_id_counter: Arc<AtomicUsize>,
}

#[derive(Debug, Fail)]
#[fail(display = "Unable to make method calls because underlying connection is closed")]
pub struct ConnectionClosed {}

impl Transport {
    pub fn new(ws_url: String) -> Result<Self, Error> {
        let (messages_tx, messages_rx) = mpsc::channel();
        let web_socket_connection = WebSocketConnection::new(&ws_url, messages_tx)?;

        let waiting_call_registry = Arc::new(WaitingCallRegistry::new());

        let listeners = Arc::new(Mutex::new(HashMap::new()));

        let open = Arc::new(AtomicBool::new(true));

        Self::handle_incoming_messages(
            messages_rx,
            Arc::clone(&waiting_call_registry),
            Arc::clone(&listeners),
            Arc::clone(&open),
        );

        Ok(Transport {
            web_socket_connection,
            waiting_call_registry,
            listeners,
            open,
            call_id_counter: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// Returns a number based on thread-safe unique counter, incrementing it so that the
    /// next CallId is different.
    pub fn unique_call_id(&self) -> CallId {
        self.call_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn call_method<C>(&self, method: C) -> Result<C::ReturnObject, Error>
    where
        C: cdtp::Method + serde::Serialize,
    {
        if !self.open.load(Ordering::SeqCst) {
            return Err(ConnectionClosed {}.into());
        }
        let call_id = self.unique_call_id();
        let call = method.to_method_call(call_id);
        let message_text = serde_json::to_string(&call).unwrap();

        let response_rx = self.waiting_call_registry.register_call(call.id);
        if let Err(e) = self.web_socket_connection.send_message(&message_text) {
            trace!("Can't send over websocket, deregistering {:?}", call.id);
            self.waiting_call_registry.unregister_call(call.id);
            return Err(e);
        }

        let response = response_rx.recv().unwrap()?;
        cdtp::parse_response::<C::ReturnObject>(response)
    }

    pub fn call_method_on_target<C>(
        &self,
        session_id: &SessionId,
        method_call: MethodCall<C>,
    ) -> Result<C::ReturnObject, Error>
    where
        C: cdtp::Method + serde::Serialize,
    {
        if !self.open.load(Ordering::SeqCst) {
            return Err(ConnectionClosed {}.into());
        }

        let message = &serde_json::to_string(&method_call).unwrap();

        let target_method = target::methods::SendMessageToTarget {
            target_id: None,
            session_id: Some(session_id.as_str()),
            message,
        };

        let response_rx = self.waiting_call_registry.register_call(method_call.id);

        if let Err(e) = self.call_method(target_method) {
            self.waiting_call_registry.unregister_call(method_call.id);
            return Err(e);
        };

        trace!("waiting for response to method call");
        let response = response_rx.recv().unwrap()?;
        trace!("received response to method call");

        let return_object = cdtp::parse_response::<C::ReturnObject>(response)?;
        Ok(return_object)
    }

    pub fn listen_to_browser_events(&self) -> Receiver<Event> {
        let (events_tx, events_rx) = mpsc::channel();

        let mut listeners = self.listeners.lock().unwrap();
        listeners.insert(ListenerId::Browser, events_tx);

        events_rx
    }

    pub fn listen_to_target_events(&self, session_id: SessionId) -> Receiver<Event> {
        let (events_tx, events_rx) = mpsc::channel();

        let mut listeners = self.listeners.lock().unwrap();
        listeners.insert(ListenerId::SessionId(session_id), events_tx);

        events_rx
    }

    fn handle_incoming_messages(
        messages_rx: Receiver<cdtp::Message>,
        waiting_call_registry: Arc<WaitingCallRegistry>,
        listeners: Listeners,
        open: Arc<AtomicBool>,
    ) {
        std::thread::spawn(move || {
            for message in messages_rx {
                match message {
                    Message::Response(response_to_browser_method_call) => {
                        waiting_call_registry.resolve_call(response_to_browser_method_call);
                    }

                    Message::Event(browser_event) => match browser_event {
                        Event::ReceivedMessageFromTarget(target_message_event) => {
                            let session_id = target_message_event.params.session_id.into();
                            let raw_message = target_message_event.params.message;

                            if let Ok(target_message) = cdtp::parse_raw_message(&raw_message) {
                                match target_message {
                                    Message::Event(target_event) => {
                                        if let Some(tx) = listeners
                                            .lock()
                                            .unwrap()
                                            .get(&ListenerId::SessionId(session_id))
                                        {
                                            tx.send(target_event)
                                                .expect("Couldn't send event to listener");
                                        }
                                    }

                                    Message::Response(resp) => {
                                        waiting_call_registry.resolve_call(resp);
                                    }
                                }
                            } else {
                                trace!(
                                    "Message from target isn't recognised: {:?}",
                                    &raw_message[..30]
                                );
                            }
                        }

                        _ => {
                            if let Some(tx) = listeners.lock().unwrap().get(&ListenerId::Browser) {
                                tx.send(browser_event).unwrap()
                            }
                        }
                    },
                }
            }

            trace!("Shutting down message handling loop");

            open.store(false, Ordering::SeqCst);
            waiting_call_registry.cancel_outstanding_method_calls();
        });
    }
}
