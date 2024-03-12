use std::thread;
use std::time::Duration;

fn main() {
    loop {
        let rng = rand::thread_rng();
        println!("{}", lipsum::lipsum_words_with_rng(rng, 15));
        thread::sleep(Duration::from_secs(1));
    }
}
