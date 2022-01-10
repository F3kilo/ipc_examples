use std::io;
use std::io::Write;

fn main() {
    let mut file = file::open_write();
    loop {
        let mut str_buf = String::new();
        io::stdin().read_line(&mut str_buf).unwrap();
        file.write_all(str_buf.as_bytes()).unwrap();
    }
}
