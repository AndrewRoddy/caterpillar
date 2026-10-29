use std::collections::HashMap;

pub fn tokenize(expression: String) -> Vec<HashMap<String, String>> {
    let mut line = 1;
    let mut column = 1;

    let mut tokens: Vec<HashMap<String, String>> = Vec::new();

    for x in expression.chars() {
        // Makes empty dictionary / hashmap
        let mut dict = HashMap::new();

        // Checks for newline
        if x == '\n' {
            line += 1;
            column = 1;
            continue;
        }

        // Checks the patterns (very bad)
        if x.to_digit(10) != None {
            dict.insert("tag".to_string(), "number".to_string());
            dict.insert("value".to_string(), x.to_string());
        } else if x == '*' {
            dict.insert("tag".to_string(), "*".to_string());
        } else if x == '+' {
            dict.insert("tag".to_string(), "+".to_string());
        } else {
            dict.insert("tag".to_string(), "None".to_string());
        }

        if !dict.contains_key("value") {
            dict.insert("value".to_string(), "null".to_string());
        }

        // Inserts the line and column counts
        dict.insert("line".to_string(), line.to_string());
        dict.insert("column".to_string(), column.to_string());

        // Runs a test print if in dev mode
        println!(
            "| line: {} | column: {} | tag: {} | value: {} |",
            dict["line"], dict["column"], dict["tag"], dict["value"],
        );

        tokens.push(dict);

        column += 1;
    }

    return tokens;
}
