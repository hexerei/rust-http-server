use std::net::{TcpListener, TcpStream};

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }
    pub fn run(self) {
        println!("Starting Server on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            let res = listener.accept();
            if res.is_err() {
                continue;
            }
                let (stream, addr) = res.unwrap();
        }

    }
}        

