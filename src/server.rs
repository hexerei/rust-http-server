use std::{net::{TcpListener, TcpStream}, io::Read};

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
                Ok((mut stream, _)) => {
                    let mut buffer = [0u8; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
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

