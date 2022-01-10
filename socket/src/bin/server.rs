use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:55331").expect("can't bind socket");
    loop {
        let mut buf = [0u8; 1024];
        socket.recv(&mut buf).expect("can't recv datagram");
        let str = String::from_utf8_lossy(&buf);
        print!("{}", str)
    }
}
