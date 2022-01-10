use file::open_read;
use std::io::Read;

fn main() {
    let mut file = open_read();
    loop {
        let mut buf = [0; 1024];
        if let Ok(red) = file.read(&mut buf) {
            let str = String::from_utf8_lossy(&buf);
            if red != 0 {
                print!("{}", str)
            }
        }
    }
}
