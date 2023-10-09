use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;
use log::{debug, info, trace, warn};
use tungstenite::http::Response;
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;
use url::Url;

use crate::types::{parse_raw_message, Message};

type TungsteniteWebsocketConnection = tungstenite::protocol::WebSocket<MaybeTlsStream<TcpStream>>;

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

pub struct WebSocketConnection {
    connection: Arc<Mutex<TungsteniteWebsocketConnection>>,
    thread: std::thread::JoinHandle<()>,
    process_id: Option<u32>,
}

// TODO websocket::sender::Writer is not :Debug...
impl std::fmt::Debug for WebSocketConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "WebSocketConnection {{}}")
    }
}

impl WebSocketConnection {
    pub fn new(
        ws_url: &Url,
        process_id: Option<u32>,
        messages_tx: mpsc::Sender<Message>,
    ) -> Result<Self> {
        let (connection, _) = Self::websocket_connection(ws_url)?;

        let connection = Arc::new(Mutex::new(connection));

        let thread = {
            let sender = connection.clone();
            std::thread::spawn(move || {
                trace!("Starting msg dispatching loop");
                Self::dispatch_incoming_messages(sender, messages_tx, process_id);
                trace!("Quit loop msg dispatching loop");
            })
        };

        Ok(Self {
            connection,
            thread,
            process_id,
        })
    }

    pub fn shutdown(&self) {
        trace!(
            "Shutting down WebSocket connection for Chrome {:?}",
            self.process_id
        );
        if let Err(err) = self.connection.lock().unwrap().close(None) {
            debug!(
                "Couldn't shut down WS connection for Chrome {:?}: {}",
                self.process_id, err
            );
        }

        self.connection.lock().unwrap().flush().ok();
        self.thread.thread().unpark();
    }

    fn dispatch_incoming_messages(
        receiver: Arc<Mutex<TungsteniteWebsocketConnection>>,
        messages_tx: mpsc::Sender<Message>,
        process_id: Option<u32>,
    ) {
        loop {
            let message = receiver.lock().unwrap().read();

            match message {
                Err(err) => match err {
                    tungstenite::Error::Io(err) => {
                        if matches!(
                            err.kind(),
                            std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                        ) {
                            std::thread::park_timeout(READ_TIMEOUT_DURATION);
                        } else {
                            debug!("WS IO Error for Chrome #{:?}: {}", process_id, err);
                            break;
                        }
                    }
                    tungstenite::Error::ConnectionClosed
                    | tungstenite::Error::AlreadyClosed
                    | tungstenite::Error::Protocol(
                        tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                    ) => break,
                    error => {
                        panic!("Unhandled WebSocket error for Chrome #{process_id:?}: {error:?}");
                    }
                },
                Ok(message) => {
                    if let tungstenite::protocol::Message::Text(message_string) = message {
                        if let Ok(message) = parse_raw_message(&message_string) {
                            if messages_tx.send(message).is_err() {
                                break;
                            }
                        } else {
                            trace!(
                                "Incoming message isn't recognised as event or method response: {}",
                                message_string
                            );
                        }
                    } else {
                        panic!("Got a weird message: {message:?}");
                    }
                }
            }
        }

        info!("Sending shutdown message to message handling loop");
        if messages_tx.send(Message::ConnectionShutdown).is_err() {
            warn!("Couldn't send message to transport loop telling it to shut down");
        }
    }

    pub fn websocket_connection(
        ws_url: &Url,
    ) -> Result<(
        tungstenite::WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    )> {
        let mut client = tungstenite::client::connect_with_config(
            ws_url,
            Some(WebSocketConfig::default()),
            u8::MAX - 1,
        )?;

        let stream = client.0.get_mut();

        // this should be handled in tungstenite
        let stream = match stream {
            MaybeTlsStream::Plain(s) => s,
            #[cfg(feature = "native-tls")]
            MaybeTlsStream::NativeTls(s) => s.get_mut(),
            #[cfg(feature = "rustls")]
            MaybeTlsStream::Rustls(s) => &mut s.sock,

            _ => todo!(),
        };
        stream.set_read_timeout(Some(READ_TIMEOUT_DURATION))?;

        debug!("Successfully connected to WebSocket: {}", ws_url);

        Ok(client)
    }

    pub fn send_message(&self, message_text: &str) -> Result<()> {
        let message = tungstenite::protocol::Message::text(message_text);
        let mut sender = self.connection.lock().unwrap();
        sender.send(message)?;
        self.thread.thread().unpark();
        Ok(())
    }
}

impl Drop for WebSocketConnection {
    fn drop(&mut self) {
        info!("dropping websocket connection");
    }
}
