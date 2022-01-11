use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:55331").expect("can't bind socket");
    loop {
        let mut buf = [0u8; 4];
        let (_, client) = socket.recv_from(&mut buf).expect("can't recv datagram");
        let num = u32::from_be_bytes(buf);
        println!("From: {}, number: {}", client.port(), num)
    }
}
