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
            match listener.accept() {
                Ok((stream, _)) => {
                    let a = 5;
                    println!("Ok")
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }

    }
}        

