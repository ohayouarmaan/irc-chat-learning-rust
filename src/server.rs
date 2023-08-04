use core::panic;
use std::convert::Infallible;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, BufReader};
use std::thread;
use std::io::Read;
use std::io::prelude::*;

use crossbeam_channel::unbounded;

use super::connection::connection;

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    connections: Vec<connection>,
    msg_sender: crossbeam_channel::Sender<connection>,
    msg_recievers: Vec<crossbeam_channel::Receiver<connection>>
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
        let (s, r) = crossbeam_channel::unbounded::<connection>();
        let socket_server: Self = Self {
            listener: bind(host, port)?,
            connections: vec![],
            msg_sender: s,
            msg_recievers: vec![r]
        };
        return Ok(socket_server);
    }

    pub fn handle_connection(msg_recv: crossbeam_channel::Receiver<connection>) {
        let value = msg_recv.recv().unwrap();
        let mut reader = BufReader::new(value.stream);
        let mut line  = String::new();
        print!("{:?}", value.host);
        while let Ok(len) = reader.read_line(&mut line) {
            if len == 0 {
                break;
            }
    
            let command_parts: Vec<&str> = line.trim_end().split_whitespace().collect();
            println!("{:?}", command_parts);
            line.clear()
        }
    }

    pub fn handle_connection_directly(&mut self, recv: crossbeam_channel::Receiver<connection>) {
        let mut value = recv.recv().unwrap();
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
                                    println!("{:?}", message_value);
                                }
                            }
                            message_value = Vec::from(&message_value[0..length]);
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
            let (ser, recv) = crossbeam_channel::unbounded::<connection>();
            // self.msg_sender.send(new_connection);
            ser.send(new_connection).unwrap();
            self.handle_connection_directly(recv);
        }
    }
}
