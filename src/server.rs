use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, BufReader};
use std::process::Command;
use std::str::from_utf8;
use std::thread;
use std::io::Read;

use super::connection::connection;
use super::rooms::room;

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    connections: Vec<connection>,
    rooms: Vec<room>
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
            rooms: vec![]
        };
        return Ok(socket_server);
    }

    fn handle_connection_directly(&mut self, recv: connection) {
        let mut value = recv;
        thread::spawn(move || {
            loop  {
                let mut message = value.recieve();
                match message {
                    Ok(msg) => {
                        if let Ok(message) = String::from_utf8(msg) {
                            let command: Vec<&str> = message.split(" ").collect();
                            match command[0] {
                                "join" => {
                                    // Join a room with the id present in command[1]
                                }
                                _ => {
                                    // Send a message saying "Invalid Command, Can't process."
                                }
                            }
                        }
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
