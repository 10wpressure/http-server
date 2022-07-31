use crate::http::Request;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {}
                                Err(e) => println!("Failed to read request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to establish a connection to the server: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a new connection: {}", e),
            }
        }
    }
}
