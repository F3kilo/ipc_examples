use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let lock = named_lock::NamedLock::create("my_named_lock").expect("can't create named lock");
    let _guard = lock.lock().expect("can't get lock");
    for i in 0..10u32 {
        socket
            .send_to(&i.to_be_bytes(), "127.0.0.1:55331")
            .expect("can't send datagram");
        thread::sleep(Duration::from_secs(1));
    }
}
