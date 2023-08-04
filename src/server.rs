use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, BufReader};
use std::thread;
use std::io::Read;

use super::connection::connection;

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    connections: Vec<connection>,
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
            connections: vec![],
        };
        return Ok(socket_server);
    }

    pub fn handle_connection_directly(&mut self, recv: connection) {
        let mut value = recv;
        thread::spawn(move || {
            loop  {
                let mut message = value.recieve();
                match message {
                    Ok(msg) => {
                        println!("New Message Recieved: {:?}", msg.len());
                    },
                    Err(e) => {
                        continue;
                    }
                }
            }
        });
    }

    pub fn accept_connections(&mut self){
        loop {
            let (tcp_stream, _addr) = self.listener.accept().unwrap();
            let new_connection = connection::new(tcp_stream);
            self.handle_connection_directly(new_connection);
        }
    }
}
