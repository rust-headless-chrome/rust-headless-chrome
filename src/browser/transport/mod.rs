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

use crate::protocol;
use crate::protocol::target;
use crate::protocol::Event;
use crate::protocol::Message;

use crate::browser::waiting_helpers::wait_for;
use crate::browser::waiting_helpers::WaitOptions;
use crate::protocol::CallId;
use std::time::Duration;
use waiting_call_registry::WaitingCallRegistry;
use web_socket_connection::WebSocketConnection;

mod waiting_call_registry;
mod web_socket_connection;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

pub enum MethodDestination {
    Target(SessionId),
    Browser,
}

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
    web_socket_connection: Arc<WebSocketConnection>,
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
        let web_socket_connection = Arc::new(WebSocketConnection::new(&ws_url, messages_tx)?);

        let waiting_call_registry = Arc::new(WaitingCallRegistry::new());

        let listeners = Arc::new(Mutex::new(HashMap::new()));

        let open = Arc::new(AtomicBool::new(true));

        Self::handle_incoming_messages(
            messages_rx,
            Arc::clone(&waiting_call_registry),
            Arc::clone(&listeners),
            Arc::clone(&open),
            Arc::clone(&web_socket_connection),
        );

        Ok(Self {
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

    pub fn call_method<C>(
        &self,
        method: C,
        destination: MethodDestination,
    ) -> Result<C::ReturnObject, Error>
    where
        C: protocol::Method + serde::Serialize,
    {
        // TODO: use get_mut to get exclusive access for entire block... maybe.
        if !self.open.load(Ordering::SeqCst) {
            return Err(ConnectionClosed {}.into());
        }
        let call_id = self.unique_call_id();
        let call = method.to_method_call(call_id);

        let message_text = serde_json::to_string(&call)?;

        let response_rx = self.waiting_call_registry.register_call(call.id);

        match destination {
            MethodDestination::Target(session_id) => {
                let target_method = target::methods::SendMessageToTarget {
                    target_id: None,
                    session_id: Some(session_id.as_str()),
                    message: &message_text,
                };
                if let Err(e) = self.call_method_on_browser(target_method) {
                    error!("Failed to call method on browser");
                    self.waiting_call_registry.unregister_call(call.id);
                    trace!("Unregistered callback: {:?}", call.id);
                    return Err(e);
                }
            }
            MethodDestination::Browser => {
                if let Err(e) = self.web_socket_connection.send_message(&message_text) {
                    self.waiting_call_registry.unregister_call(call.id);
                    return Err(e);
                } else {
                    trace!("sent method call to browser via websocket");
                }
            }
        }

        trace!("waiting for response from call registry");
        let response_result = wait_for(
            || response_rx.try_recv().ok(),
            WaitOptions {
                timeout_ms: 5000,
                sleep_ms: 10,
            },
        );
        protocol::parse_response::<C::ReturnObject>((response_result?)?)
    }

    pub fn call_method_on_target<C>(
        &self,
        session_id: SessionId,
        method: C,
    ) -> Result<C::ReturnObject, Error>
    where
        C: protocol::Method + serde::Serialize,
    {
        // TODO: remove clone
        self.call_method(method, MethodDestination::Target(session_id))
    }

    pub fn call_method_on_browser<C>(&self, method: C) -> Result<C::ReturnObject, Error>
    where
        C: protocol::Method + serde::Serialize,
    {
        self.call_method(method, MethodDestination::Browser)
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
        messages_rx: Receiver<protocol::Message>,
        waiting_call_registry: Arc<WaitingCallRegistry>,
        listeners: Listeners,
        open: Arc<AtomicBool>,
        conn: Arc<WebSocketConnection>,
    ) {
        trace!("Starting handle_incoming_messages");
        std::thread::spawn(move || {
            trace!("Inside handle_incoming_messages thread");
            // this iterator calls .recv() under the hood, so can block thread forever
            // hence need for Connection Shutdown
            loop {
                match messages_rx.recv_timeout(Duration::from_millis(20_000)) {
                    Err(_) => {
                        break;
                    }
                    Ok(message) => {
                        trace!("{:?}", message);
                        match message {
                            Message::ConnectionShutdown => {
                                trace!("Received shutdown message");
                                break;
                            }
                            Message::Response(response_to_browser_method_call) => {
                                if waiting_call_registry
                                    .resolve_call(response_to_browser_method_call)
                                    .is_err()
                                {
                                    warn!("The browser registered a call but then closed its receiving channel");
                                    break;
                                }
                            }

                            Message::Event(browser_event) => match browser_event {
                                Event::ReceivedMessageFromTarget(target_message_event) => {
                                    let session_id = target_message_event.params.session_id.into();
                                    let raw_message = target_message_event.params.message;

                                    if let Ok(target_message) =
                                        protocol::parse_raw_message(&raw_message)
                                    {
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
                                                if waiting_call_registry.resolve_call(resp).is_err()
                                                {
                                                    warn!("The browser registered a call but then closed its receiving channel");
                                                    break;
                                                }
                                            }
                                            Message::ConnectionShutdown => {}
                                        }
                                    } else {
                                        trace!(
                                            "Message from target isn't recognised: {:?}",
                                            &raw_message[..30]
                                        );
                                    }
                                }

                                _ => {
                                    if let Some(tx) =
                                        listeners.lock().unwrap().get(&ListenerId::Browser)
                                    {
                                        if tx.send(browser_event).is_err() {
                                            trace!("Couldn't send browser an event");
                                            break;
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }

            trace!("Shutting down message handling loop");

            // Need to do this because otherwise WS thread might block forever
            conn.shutdown();

            open.store(false, Ordering::SeqCst);
            waiting_call_registry.cancel_outstanding_method_calls();
            let mut listeners = listeners.lock().unwrap();
            *listeners = HashMap::new();
            trace!("cleared listeners, I think");
        });
    }
}
