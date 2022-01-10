use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::path::Path;

pub fn open_write() -> File {
    let path = Path::new("target/temp/msgs.txt");
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).expect("can't create directory");
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("can't open file for write")
}

pub fn open_read() -> File {
    let path = Path::new("target/temp/msgs.txt");
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).expect("can't create directory");
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("can't open file for read");
    file.seek(SeekFrom::End(0)).expect("can't seek read file");
    file
}
