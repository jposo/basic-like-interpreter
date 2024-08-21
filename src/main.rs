mod scanner;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/buzz.jpo")
        .expect("Should haave been able to read the file");
    
    scanner::scan(&contents);
}
