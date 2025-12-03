use std::net::Ipv4Addr;
use crate::server::Server;

mod connection;
mod server;

fn main() {
    if std::env::args().len() != 3 {
        eprintln!("Invalid number of arguments!");
        println!("Usage: concord [server OR client] PORT");
    }

    let port = std::env::args().nth(2).unwrap().parse::<u16>().unwrap();

    if std::env::args().nth(1) == Some("server".into())  {
        let mut server = Server::new(Ipv4Addr::new(127, 0, 0, 1), port);
        server.run();
    }
}