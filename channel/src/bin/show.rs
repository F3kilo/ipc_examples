use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let chars = get_chars(&line);
        for (char, cnt) in chars {
            print!("'{}': {}; ", char, cnt);
        }
            println!()
    }
}

fn get_chars(s: &str) -> HashMap<char, u32> {
    let mut hash_map = HashMap::new();
    for c in s.trim().chars() {
        *hash_map.entry(c).or_default() += 1;
    }
    hash_map
}
