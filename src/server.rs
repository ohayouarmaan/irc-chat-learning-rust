use std::net::{TcpListener, TcpStream};
use std::io::Error;
use std::thread;

use super::connection;

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    connections: Vec<connection::connection>,
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

    pub fn handle_connection(stream: &connection::connection) {
        println!("Are we reaching here?");
        println!("{:?}", stream.host);
    }

    pub fn accept_connections(&mut self){
        for stream in self.listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    let new_connection = connection::connection::new(tcp_stream);
                    thread::spawn(move || {
                        IRCServer::handle_connection(&new_connection);
                    });

                },
                _ => panic!("Error occured while connecting.")
            };
        };
    }
}
