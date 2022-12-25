use std::sync::mpsc;
use std::sync::Arc;

use anyhow::Result;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt,
};
use futures_util::stream::StreamExt;
use log::*;
use tokio::{net::TcpStream, runtime::Runtime, sync::Mutex};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::client::IntoClientRequest;
use url::Url;

use crate::types::{parse_raw_message, Message};

#[derive(Debug)]
pub struct WebSocketConnection {
    sender: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>>>,
    runtime: Arc<Runtime>,
    process_id: Option<u32>,
}

impl WebSocketConnection {
    pub fn new(
        ws_url: &Url,
        process_id: Option<u32>,
        messages_tx: mpsc::Sender<Message>,
    ) -> Result<Self> {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?,
        );

        let connection: Result<_> = runtime.block_on(async {
            let (stream, _) = connect_async(ws_url.to_string().into_client_request()?).await?;

            debug!("Successfully connected to WebSocket: {}", ws_url);

            Ok(stream)
        });

        let (sender, recv) = connection?.split();

        let clone_rt = runtime.clone();
        std::thread::spawn(move || {
            trace!("Starting msg dispatching loop");
            clone_rt.block_on(Self::dispatch_incoming_messages(
                recv,
                messages_tx,
                process_id,
            ));
            trace!("Quit loop msg dispatching loop");
        });

        Ok(Self {
            sender: Arc::new(Mutex::new(sender)),
            runtime,
            process_id,
        })
    }

    pub fn shutdown(&self) {
        trace!(
            "Shutting down WebSocket connection for Chrome {:?}",
            self.process_id
        );

        self.runtime.block_on(async {
            if self.sender.lock().await.close().await.is_err() {
                debug!(
                    "Couldn't shut down WS connection for Chrome {:?}",
                    self.process_id
                );
            }
        });
    }

    async fn dispatch_incoming_messages(
        mut receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        messages_tx: mpsc::Sender<Message>,
        process_id: Option<u32>,
    ) {
        loop {
            let ws_message = receiver.next().await;
            match ws_message {
                Some(Err(error)) => match error {
                    tungstenite::Error::Io(err) => {
                        debug!("WS IO Error for Chrome #{:?}: {}", process_id, err);
                        break;
                    }
                    _ => panic!(
                        "Unhandled WebSocket error for Chrome #{:?}: {:?}",
                        process_id, error
                    ),
                },
                Some(Ok(message)) => {
                    if let tungstenite::Message::Text(message_string) = message {
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
                        panic!("Got a weird message: {:?}", message);
                    }
                }
                // Keep waiting for an event in case nothing was fetched yet.
                None => (),
            }
        }

        info!("Sending shutdown message to message handling loop");
        if messages_tx.send(Message::ConnectionShutdown).is_err() {
            warn!("Couldn't send message to transport loop telling it to shut down")
        }
    }

    pub fn send_message(&self, message_text: &str) -> Result<()> {
        let message = tungstenite::Message::text(message_text);

        Ok(self
            .runtime
            .block_on(async { self.sender.lock().await.send(message).await })?)
    }
}

impl Drop for WebSocketConnection {
    fn drop(&mut self) {
        info!("dropping websocket connection");
    }
}
