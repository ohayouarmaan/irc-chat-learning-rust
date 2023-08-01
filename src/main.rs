use std::net::{TcpListener, TcpStream};

mod server;
fn main() {
    let listener: Result<server::IRCServer, String> = server::IRCServer::new("0.0.0.0", 8000);
    let main_irc_server = match listener {
        Ok(main_server) => main_server,
        _ => panic!("Error occured while connecting to the address.")
    };
    println!("{:?}", main_irc_server);
}
