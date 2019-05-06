use std::fs;
use std::io::Read;

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn main() {
    println!("Hello, world!");
}
