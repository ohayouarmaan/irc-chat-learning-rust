use std::net::{TcpListener, TcpStream};
use std::io::Error;

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    connections: Vec<TcpStream>,
}
pub fn bind(host: &str, port: i32) -> Result<TcpListener, String> {
    let listener: Result<TcpListener, std::io::Error> = TcpListener::bind(format!("{}:{}", host, port));
    match listener {
        Ok(listener) => return Ok(listener),
        _ => return Err("Something's wrong binding to the port".to_owned())
    }
}

impl IRCServer {
    pub fn new(host: &str, port: i32) -> Result<Self, String> {
        let socket_server: Self = Self {
            listener: bind(host, port)?,
            connections: vec![]
        };
        return Ok(socket_server);
    }

    pub fn accept_connections(&mut self){
        for stream in self.listener.incoming() {
            match stream {
                Ok(tcp_stream) => self.connections.push(tcp_stream),
                _ => panic!("Error occured while connecting.")
            };
        };
    }
}
