use core::panic;
use std::net::{TcpStream, IpAddr};
use crossbeam_channel;
use std::io::Read;
use std::thread;


#[derive(Debug )]
pub struct connection {
    pub stream: TcpStream,
    pub host: IpAddr,
    pub server: crossbeam_channel::Sender<Vec<u8>>,
    pub reciever: crossbeam_channel::Receiver<Vec<u8>>
}

impl connection {
    pub fn new(stream: TcpStream) -> Self {
        let ip = stream.local_addr().unwrap().ip();
        let (mut server, reciever) = crossbeam_channel::unbounded::<Vec<u8>>();
        return Self {
            stream,
            host: ip,
            server,
            reciever
        }
    }
    pub fn recieve(&mut self) -> Result<Vec<u8>, String> {
        let mut line = [0u8; 1024];
        let msg = self.stream.read(&mut line).unwrap();
        if msg != 0 {
            let mut line = line.to_vec();

            let final_message: Vec<u8> = match String::from_utf8(line.clone()) {
                Ok(mut s) => {
                    s = s.replace("\0", "");
                    let mut message_value: Vec<u8> = Vec::new();
                    let length = s.trim_end().parse::<usize>().unwrap();
                    println!("length: {}", length);
                    let mut buffer = [0u8; 1024];
                    if (length / 1024) >= 1 {
                        loop {
                            let recieved_value = self.stream.read(&mut buffer).unwrap();
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
                        let recieved_value = self.stream.read(&mut buffer).unwrap();
                        if recieved_value != 0 {
                            for b in buffer {
                                message_value.push(b);
                            }
                            buffer = [0u8; 1024];
                        }
                    }
                    message_value = Vec::from(&message_value[0..length]);
                    println!("MESSAGE RECIEVED OF LENGTH: {}", message_value.len());
                    line.clear();
                    message_value
                },
                _ => {
                    panic!("wtf");
                }
            };
            return Ok(final_message);
        } else {
            return Err(String::from("Message overflow"));
        }
    }
}
