use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

#[derive(Clone)]
pub enum Message {
    Connect(String),
    Disconnect(String),
    Data(String),
}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub(crate) fn new(ip: Ipv4Addr, port: u16) -> Connection {
        let stream = TcpStream::connect((ip, port)).unwrap();
        stream.set_nonblocking(true).unwrap();

        Connection { stream }
    }

    pub(crate) fn from_stream(stream: TcpStream) -> Connection {
        stream.set_nonblocking(true).unwrap();

        Connection { stream }
    }

    pub fn send(&mut self, msg: &Message) {
        match msg {
            Message::Connect(data) => self.write(format!("CONN:{data}")),
            Message::Disconnect(data) => self.write(format!("DISC:{data}")),
            Message::Data(data) => self.write(format!("DATA:{data}")),
        }
    }

    pub fn recv(&mut self) -> Option<Message> {
        let str = self.read();
        if let Some(str) = str {
            if str.starts_with("CONN:") {
                return Some(Message::Connect(str[5..].to_string()));
            }

            if str.starts_with("DISC:") {
                return Some(Message::Disconnect(str[5..].to_string()));
            }

            if str.starts_with("DATA:") {
                return Some(Message::Data(str[5..].to_string()));
            }
        }

        None
    }

    fn write(&mut self, data: String) {
        let vec = data.as_bytes();
        self.stream.write_all(vec).unwrap();
    }

    fn read(&mut self) -> Option<String> {
        let buf: &mut [u8; 1024] = &mut [0; 1024];
        match self.stream.read(buf) {
            Ok(0) => None,
            Ok(n) => Some(String::from_utf8_lossy(&buf[0..n]).to_string()),
            Err(_) => None,
        }
    }
}
