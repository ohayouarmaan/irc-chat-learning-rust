use std::net::{TcpStream, IpAddr};

#[derive(Debug )]
pub struct connection {
    pub stream: TcpStream,
    pub host: IpAddr
}

impl connection {
    pub fn new(stream: TcpStream) -> Self {
        let ip = stream.local_addr().unwrap().ip();
        return Self {
            stream,
            host: ip
        }
    }
}
