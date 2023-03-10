use std::net::TcpListener;
use std::io::Read;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("Starting Server on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0u8; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                    //let mut buf = String::new();
                    //stream.read_to_string(&mut buf)
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }

    }
}

