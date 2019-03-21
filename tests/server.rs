use std::sync::{atomic, Arc};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{fs, io};

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

    #[allow(dead_code)]
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port())
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

fn basic_http_response<'a>(
    body: &'a str,
    content_type: &'static str,
) -> tiny_http::Response<&'a [u8]> {
    tiny_http::Response::new(
        200.into(),
        vec![tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap()],
        body.as_bytes(),
        Some(body.len()),
        None,
    )
}

#[allow(dead_code)]
fn not_found_response() -> tiny_http::Response<io::Empty> {
    tiny_http::Response::new_empty(404.into())
}

#[allow(dead_code)]
pub fn file_server(path: &'static str) -> Server {
    Server::new(move |request: tiny_http::Request| {
        let url = if request.url() == "/" {
            "/index.html"
        } else {
            request.url()
        };

        let file_path = format!("{}{}", path, url);

        if let Ok(file_contents) = fs::read_to_string(file_path) {
            let content_type = if url.ends_with(".js") {
                "application/javascript"
            } else if url.ends_with(".css") {
                "text/css"
            } else {
                "text/html"
            };

            let response = basic_http_response(&file_contents, content_type);
            request.respond(response)
        } else {
            let response = not_found_response();
            request.respond(response)
        }
    })
}
