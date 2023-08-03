use std::{net::{TcpListener, TcpStream}, thread};

mod server;
mod connection;
mod Server;


fn main() {
    let listener: Result<server::IRCServer, String> = server::IRCServer::new("0.0.0.0", 8000);
    let mut main_irc_server = match listener {
        Ok(main_server) => main_server,
        _ => panic!("Error occured while connecting to the address.")
    };
    server::IRCServer::accept_connections(&mut main_irc_server);
}
