use std::collections::HashMap;
use std::fs;

fn main() {
    // Reads data from file
    let expression = fs::read_to_string("src/example.v").expect("Failed to read file");

    let tokens = tokenize(expression);
    println!("{tokens}");
}

fn tokenize(expression: String) -> String {
    let mut patterns = HashMap::new();
    patterns.insert(String::from(r"\s+"), String::from("whitespace"));
    let value = &patterns[r"\s+"];

    println!("{expression}");
    println!("{value}");

    return expression;
}
