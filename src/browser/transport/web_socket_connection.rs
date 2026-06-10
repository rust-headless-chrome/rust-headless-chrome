use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, Once};

use anyhow::Result;
use log::{debug, info, trace, warn};
use tungstenite::http::Response;
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;
use url::Url;

use crate::types::{Message, parse_raw_message};

type TungsteniteWebsocketConnection = tungstenite::protocol::WebSocket<MaybeTlsStream<TcpStream>>;

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

#[cfg(feature = "rustls-tls-webpki-roots")]
static RUSTLS_INIT: Once = Once::new();

#[cfg(feature = "rustls-tls-webpki-roots")]
fn init_rustls_provider() {
    RUSTLS_INIT.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

#[cfg(feature = "rustls-tls-webpki-roots")]
fn add_root_certificates(
    roots: &mut rustls::RootCertStore,
    root_cert: Option<&[u8]>,
) -> Result<()> {
    use std::io::Cursor;

    let Some(cert_bytes) = root_cert else {
        return Ok(());
    };

    if cert_bytes.starts_with(b"-----BEGIN CERTIFICATE-----") {
        let mut reader = Cursor::new(cert_bytes);

        for cert in rustls_pemfile::certs(&mut reader) {
            roots.add(cert?)?;
        }
    } else {
        roots.add(rustls::pki_types::CertificateDer::from(cert_bytes.to_vec()))?;
    }

    Ok(())
}

fn set_read_timeout(stream: &mut MaybeTlsStream<TcpStream>) -> Result<()> {
    let tcp_stream = match stream {
        MaybeTlsStream::Plain(s) => s,

        #[cfg(any(
            feature = "rustls-tls-native-roots",
            feature = "rustls-tls-webpki-roots"
        ))]
        MaybeTlsStream::Rustls(s) => &mut s.sock,

        #[cfg(feature = "native-tls")]
        MaybeTlsStream::NativeTls(s) => s.get_mut(),

        #[allow(unreachable_patterns)]
        _ => {
            return Err(anyhow::anyhow!("unsupported websocket stream type"));
        }
    };

    tcp_stream.set_read_timeout(Some(READ_TIMEOUT_DURATION))?;

    Ok(())
}

pub struct WebSocketConnection {
    connection: Arc<Mutex<TungsteniteWebsocketConnection>>,
    thread: std::thread::JoinHandle<()>,
    process_id: Option<u32>,
}

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
        root_cert: Option<Vec<u8>>,
    ) -> Result<Self> {
        let (connection, _) =
            Self::websocket_connection_with_root_cert(ws_url, root_cert.as_deref())?;

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

        if let Ok(mut connection) = self.connection.lock() {
            if let Err(err) = connection.close(None) {
                debug!(
                    "Couldn't shut down WS connection for Chrome {:?}: {}",
                    self.process_id, err
                );
            }

            connection.flush().ok();
        }

        self.thread.thread().unpark();
    }

    fn dispatch_incoming_messages(
        receiver: Arc<Mutex<TungsteniteWebsocketConnection>>,
        messages_tx: mpsc::Sender<Message>,
        process_id: Option<u32>,
    ) {
        loop {
            let message = match receiver.lock() {
                Ok(mut receiver) => receiver.read(),
                Err(err) => {
                    debug!("WS mutex poisoned for Chrome #{process_id:?}: {err}");
                    break;
                }
            };

            match message {
                Err(err) => match err {
                    tungstenite::Error::Io(err) => {
                        if matches!(
                            err.kind(),
                            std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                        ) {
                            std::thread::park_timeout(READ_TIMEOUT_DURATION);
                        } else {
                            debug!("WS IO Error for Chrome #{process_id:?}: {err}");
                            break;
                        }
                    }
                    tungstenite::Error::ConnectionClosed
                    | tungstenite::Error::AlreadyClosed
                    | tungstenite::Error::Protocol(
                        tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                    ) => break,
                    error => {
                        debug!("Unhandled WebSocket error for Chrome #{process_id:?}: {error:?}");
                        break;
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
                                "Incoming message isn't recognised as event or method response: {message_string}",
                            );
                        }
                    } else if let tungstenite::protocol::Message::Close(close_frame) = message {
                        match close_frame {
                            Some(tungstenite::protocol::CloseFrame { code, reason }) => {
                                debug!(
                                    "Received close frame from Chrome #{process_id:?}: {code:?} {reason:?}",
                                );

                                if code != tungstenite::protocol::frame::coding::CloseCode::Normal {
                                    debug!("Abnormal close code {code:?}, shutting down");
                                }
                            }
                            None => {
                                debug!("Received close frame from Chrome #{process_id:?}: None");
                            }
                        }

                        break;
                    } else {
                        debug!("Ignoring unexpected WebSocket message: {message:?}");
                    }
                }
            }
        }

        info!("Sending shutdown message to message handling loop");

        if messages_tx.send(Message::ConnectionShutdown).is_err() {
            warn!("Couldn't send message to transport loop telling it to shut down");
        }
    }

    pub fn websocket_connection_with_root_cert(
        ws_url: &Url,
        root_cert: Option<&[u8]>,
    ) -> Result<(
        tungstenite::WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    )> {
        let config = Some(
            WebSocketConfig::default()
                .accept_unmasked_frames(true)
                .max_message_size(None)
                .max_frame_size(None),
        );

        if root_cert.is_none() {
            let mut client =
                tungstenite::client::connect_with_config(ws_url.as_str(), config, u8::MAX - 1)?;

            set_read_timeout(client.0.get_mut())?;

            debug!("Successfully connected to WebSocket: {ws_url}");

            return Ok(client);
        }

        #[cfg(feature = "rustls-tls-webpki-roots")]
        {
            use tungstenite::client::IntoClientRequest;

            init_rustls_provider();

            let host = ws_url
                .host_str()
                .ok_or_else(|| anyhow::anyhow!("missing websocket host: {ws_url}"))?;

            let port = ws_url
                .port_or_known_default()
                .ok_or_else(|| anyhow::anyhow!("missing websocket port: {ws_url}"))?;

            let tcp = TcpStream::connect((host, port))?;

            let mut roots = rustls::RootCertStore::empty();
            add_root_certificates(&mut roots, root_cert)?;

            let tls_config = rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();

            let connector = tungstenite::Connector::Rustls(Arc::new(tls_config));
            let request = ws_url.as_str().into_client_request()?;

            let mut client =
                tungstenite::client_tls_with_config(request, tcp, config, Some(connector))?;

            set_read_timeout(client.0.get_mut())?;

            debug!("Successfully connected to WebSocket with custom root cert: {ws_url}");

            Ok(client)
        }

        #[cfg(not(feature = "rustls-tls-webpki-roots"))]
        {
            Err(anyhow::anyhow!(
                "root_cert was provided, but feature rustls-tls-webpki-roots is not enabled"
            ))
        }
    }

    pub fn send_message(&self, message_text: &str) -> Result<()> {
        let message = tungstenite::protocol::Message::text(message_text);

        let mut sender = self
            .connection
            .lock()
            .map_err(|err| anyhow::anyhow!("WS mutex poisoned: {err}"))?;

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
