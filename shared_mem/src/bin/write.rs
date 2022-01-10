use shared_memory::Shmem;
use std::io;

fn main() {
    let mut shmem = create_or_open();
    loop {
        let mut str_buf = String::new();
        io::stdin().read_line(&mut str_buf).unwrap();
        if !str_buf.trim().is_empty() {
            let bytes = str_buf.trim().as_bytes();
            let slice = unsafe { shmem.as_slice_mut() };
            let msg_len = bytes.len() as u32;
            slice[..4].copy_from_slice(&msg_len.to_be_bytes());
            slice[4..4 + bytes.len()].copy_from_slice(bytes);
        }
    }
}

fn create_or_open() -> Shmem {
    shared_memory::ShmemConf::default()
        .size(1024)
        .os_id("my_shmem")
        .open()
        .unwrap_or_else(|_| {
            shared_memory::ShmemConf::default()
                .size(1024)
                .os_id("my_shmem")
                .create()
                .unwrap()
        })
}
