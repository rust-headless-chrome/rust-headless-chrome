use std::io;
use std::sync::{atomic, Arc};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Server {
    server: Arc<tiny_http::Server>,
    handler: Option<JoinHandle<Result<(), io::Error>>>,
    shall_exit: Arc<atomic::AtomicBool>,
}

impl Server {
    pub fn new(
        mut responder: impl FnMut(tiny_http::Request) -> Result<(), io::Error> + Send + 'static,
    ) -> Self {
        let server = Arc::new(tiny_http::Server::http("127.0.0.1:0").unwrap());
        let shall_exit = Arc::new(atomic::AtomicBool::new(false));
        let srv = server.clone();
        let exit = shall_exit.clone();
        let handler = std::thread::spawn(move || {
            loop {
                if let Some(r) = srv.recv_timeout(Duration::from_millis(1000))? {
                    responder(r)?;
                }
                if exit.load(atomic::Ordering::Relaxed) {
                    break;
                }
            }
            Ok(())
        });
        Server {
            server,
            handler: Some(handler),
            shall_exit,
        }
    }

    #[allow(dead_code)]
    pub fn with_dumb_html(data: &'static str) -> Self {
        let responder = move |r: tiny_http::Request| {
            let response = tiny_http::Response::new(
                200.into(),
                vec![
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                ],
                io::Cursor::new(data),
                Some(data.len()),
                None,
            );
            r.respond(response)
        };
        Self::new(responder)
    }

    pub fn port(&self) -> u16 {
        self.server.server_addr().port()
    }

    pub fn exit(&mut self) -> Result<(), io::Error> {
        self.shall_exit.store(true, atomic::Ordering::Relaxed);
        match self.handler.take() {
            Some(h) => h.join().unwrap(),
            None => Ok(()),
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.exit().unwrap()
    }
}
