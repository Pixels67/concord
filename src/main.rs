mod client;
mod connection;
mod server;

use crate::client::Client;
use crate::server::Server;
use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {
    if std::env::args().len() != 3 && std::env::args().len() != 5 {
        eprintln!("Invalid number of arguments!");
        println!("Usage:");
        println!("concord server PORT");
        println!("OR");
        println!("concord client PORT NAME SERVER_IP");
    }

    let port = std::env::args().nth(2).unwrap().parse::<u16>().unwrap();

    if std::env::args().nth(1) == Some("server".into()) {
        let mut server = Server::new(Ipv4Addr::new(127, 0, 0, 1), port);
        server.run();
    }

    if std::env::args().nth(1) == Some("client".into()) {
        let name = std::env::args().nth(3).unwrap();
        let ip = Ipv4Addr::from_str(std::env::args().nth(4).unwrap().as_str()).unwrap();
        let mut client = Client::new(name, ip, port);
        client.run();
    }
}
