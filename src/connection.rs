use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub enum Message {
    Connect(String),
    Disconnect(String),
    Data(String),
}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn new(ip: Ipv4Addr, port: u16) -> Connection {
        Connection {
            stream: TcpStream::connect((ip, port)).unwrap(),
        }
    }

    pub(crate) fn from_stream(stream: TcpStream) -> Connection {
        Connection { stream }
    }

    pub fn send(&mut self, msg: &Message) {
        match msg {
            Message::Connect(data) => self.write(format!("CONN:{data}\n")),
            Message::Disconnect(data) => self.write(format!("DISC:{data}\n")),
            Message::Data(data) => self.write(format!("DATA:{data}\n")),
        }
    }

    pub fn recv(&mut self) -> Option<Message> {
        let str = self.read();
        if let Some(str) = str {
            if str.starts_with("CONN:") {
                return Some(Message::Connect(str[6..].to_string()));
            }

            if str.starts_with("DISC:") {
                return Some(Message::Disconnect(str[6..].to_string()));
            }

            if str.starts_with("DATA:") {
                return Some(Message::Data(str[6..].to_string()));
            }
        }

        None
    }

    fn write(&mut self, data: String) {
        let vec = data.as_bytes();
        _ = self.stream.write(vec).unwrap();
    }

    fn read(&mut self) -> Option<String> {
        let buf: &mut [u8; 1024] = &mut [0; 1024];
        match self.stream.read(buf).unwrap() {
            0 => None,
            _ => Some(String::from_utf8_lossy(buf).to_string()),
        }
    }
}
