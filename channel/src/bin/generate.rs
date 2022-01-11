use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("{}", lipsum::lipsum_words(15));
        thread::sleep(Duration::from_secs(1));
    }
}
