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
            loop {
                let mut line = [0u8; 1024];
                let msg = value.stream.read(&mut line).unwrap();
                if msg != 0 {
                    let mut line = line.to_vec();

                    match String::from_utf8(line.clone()) {
                        Ok(mut s) => {
                            s = s.replace("\0", "");
                            let mut message_value: Vec<u8> = Vec::new();
                            let length = s.trim_end().parse::<usize>().unwrap();
                            println!("length: {}", length);
                            let mut buffer = [0u8; 1024];
                            if (length / 1024) >= 1 {
                                loop {
                                    let recieved_value = value.stream.read(&mut buffer).unwrap();
                                    if recieved_value != 0 {
                                        for b in buffer {
                                            message_value.push(b);
                                        }
                                        buffer = [0u8; 1024];
                                        if (length) <= message_value.len() {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                let recieved_value = value.stream.read(&mut buffer).unwrap();
                                if recieved_value != 0 {
                                    for b in buffer {
                                        message_value.push(b);
                                    }
                                    buffer = [0u8; 1024];
                                }
                            }
                            message_value = Vec::from(&message_value[0..length]);
                            println!("MESSAGE RECIEVED OF LENGTH: {}", message_value.len());
                        },
                        _ => {}
                    }
                    line.clear();
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
