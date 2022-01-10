use std::io;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:55332").expect("can't bind socket");
    loop {
        let mut str = String::new();
        io::stdin().read_line(&mut str).unwrap();
        socket
            .send_to(str.as_bytes(), "127.0.0.1:55331")
            .expect("can't send datagram");
    }
}
