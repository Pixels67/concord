use crate::connection::{Connection, Message};
use std::collections::HashMap;
use std::net::{Ipv4Addr, TcpListener};

pub struct Server {
    listener: TcpListener,
    connections: Vec<Connection>,
    users: HashMap<String, usize>,
    is_running: bool,
}

impl Server {
    pub fn new(ip: Ipv4Addr, port: u16) -> Server {
        let listener = TcpListener::bind((ip, port)).unwrap();
        listener.set_nonblocking(true).unwrap();

        Server {
            listener,
            connections: vec![],
            users: HashMap::new(),
            is_running: false,
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            self.handle_incoming();
            self.handle_connections();
        }
    }

    fn handle_connections(&mut self) {
        for idx in 0..self.connections.len() {
            self.handle_connection(idx);
        }
    }

    fn handle_incoming(&mut self) {
        if let Ok((stream, addr)) = self.listener.accept() {
            println!("New connection from: {}", addr);
            self.connections.push(Connection::from_stream(stream));
        }
    }

    fn handle_connection(&mut self, index: usize) {
        let msg = self.connections[index].recv();

        if let Some(Message::Connect(name)) = &msg {
            println!("New connection: {}", name);
            self.users.insert(name.clone(), index);
        }

        if let Some(Message::Disconnect(name)) = &msg {
            println!("User disconnected: {}", name);
            self.users.remove(name);
        }

        if let Some(name) = self.get_username(index)
            && let Some(Message::Data(msg)) = msg
        {
            for (_, id) in self.users.iter() {
                if *id == index {
                    continue;
                }

                let msg = format!("{}: {}", &name, &msg);
                self.connections[*id].send(&Message::Data(msg));
            }
        }
    }

    fn get_username(&self, index: usize) -> Option<String> {
        for (name, id) in self.users.iter() {
            if *id == index {
                return Some(name.clone());
            }
        }

        None
    }
}
