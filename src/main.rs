use std::collections::HashMap;
use std::fs;

const DEV: bool = true;

fn main() {
    // Reads data from file
    let expression = fs::read_to_string("src/example.v").expect("Failed to read file");

    let tokens = tokenize(expression);
    println!("{tokens}");
}

// I think I might be going about this wrong. My first goal should be to just implement addition
// and subtraction maybe even without the regex, then slowly move to needing the regex. This will
// allow me to truly understand and learn rust without having to deal with basically needing to ask
// the chatbot.

fn tokenize(expression: String) -> String {
    let mut line = 1;
    let mut column = 1;
    let mut current_tag = String::new();

    let mut tokens: Vec<HashMap<String, String>> = Vec::new();

    println!("Test");

    for x in expression.chars() {
        // Makes empty dictionary / hashmap
        let mut dict = HashMap::new();

        // Checks the patterns (very bad)
        if x == '*' {
            dict.insert("tag".to_string(), "*".to_string());
        } else if x == '+' {
            dict.insert("tag".to_string(), "+".to_string());
        } else {
            dict.insert("tag".to_string(), "number".to_string());
        }

        // Inserts the line and column counts
        dict.insert("line".to_string(), line.to_string());
        dict.insert("column".to_string(), column.to_string());

        // Runs a test print if in dev mode
        if DEV {
            println!(
                "| line: {} | column: {} | tag: {} ",
                dict["line"], dict["column"], dict["tag"],
            );
        }
        tokens.push(dict);
    }

    return current_tag;
}
