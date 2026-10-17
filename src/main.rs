use std::fs;

fn main() {
    // Reads data from file
    let data = fs::read_to_string("src/example.v").expect("Failed to read file");
    println!("{data}");
}
