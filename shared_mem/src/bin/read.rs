use std::thread;
use std::time::Duration;

fn main() {
    let shmem = shared_memory::ShmemConf::default()
        .size(1024)
        .os_id("my_shmem")
        .open()
        .unwrap();
    loop {
        let data = unsafe { shmem.as_slice() };
        let mut len_buf = [0; 4];
        len_buf.copy_from_slice(&data[..4]);
        let msg_len = u32::from_be_bytes(len_buf) as usize;
        let msg = String::from_utf8_lossy(&data[4..4 + msg_len as usize]);
        println!("{}", msg);
        thread::sleep(Duration::from_secs(1));
    }
}
