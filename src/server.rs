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
        let mut value = msg_recv.recv().unwrap();
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

    pub fn handle_connection_directly(&mut self) {
        let mut value = self.msg_recievers[0].recv().unwrap();
        thread::spawn(move || {
            let mut line = [0u8; 1024];
            loop {
                let msg = value.stream.read(&mut line).unwrap();
                if(msg != 0) {
                    let mut line = line.to_vec();

                    match String::from_utf8(line.clone()) {
                        Ok(mut s) => {
                            println!("{:?}", s.trim_end().split_whitespace().collect::<Vec<&str>>());
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
            let (mut tcp_stream, addr) = self.listener.accept().unwrap();
            let mut new_connection = connection::new(tcp_stream);
            self.msg_sender.send(new_connection);
            self.handle_connection_directly();
        }
    }
}
