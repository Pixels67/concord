use crate::connection::{Connection, Message};
use std::net::Ipv4Addr;
use std::sync::mpsc;
use std::thread::JoinHandle;

pub struct Client {
    name: String,
    connection: Connection,
    is_running: bool,
    input_thread: JoinHandle<()>,
    input: mpsc::Receiver<String>,
}

impl Client {
    pub fn new(name: String, ip: Ipv4Addr, port: u16) -> Client {
        let (send, recv) = mpsc::channel();
        let input_thread = std::thread::spawn(move || send.send(Self::get_input()).unwrap());
        let input = recv;

        Client {
            name,
            connection: Connection::new(ip, port),
            is_running: false,
            input_thread,
            input,
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;
        self.connection
            .send(&Message::Connect(self.name.to_string()));

        while self.is_running {
            self.handle_input();
            self.render();
            //std::thread::sleep(Duration::from_millis(20));
        }

        self.connection
            .send(&Message::Disconnect(self.name.to_string()));
    }

    fn handle_input(&mut self) {
        let mut str;
        if self.input_thread.is_finished() {
            self.spawn_input_thread();
        }

        match self.input.try_recv() {
            Ok(string) => str = string,
            Err(_) => return,
        }

        if str.trim() == "/exit" {
            self.is_running = false;
            return;
        }

        str.pop();
        self.connection.send(&Message::Data(str));
    }

    fn render(&mut self) {
        if let Some(Message::Data(msg)) = self.connection.recv() {
            println!("{}", msg);
        }
    }

    fn spawn_input_thread(&mut self) {
        let (send, recv) = mpsc::channel();
        self.input_thread = std::thread::spawn(move || send.send(Self::get_input()).unwrap());
        self.input = recv;
    }

    fn get_input() -> String {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        str
    }
}
