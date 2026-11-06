use monster_regex::{Flags, Regex};
use std::collections::HashMap;
use std::sync::LazyLock;

const TEST: bool = true;

fn get_patterns() -> Vec<Vec<String>> {
    let patterns: Vec<Vec<String>> = LazyLock::new(|| {
        vec![
            vec![r"^\s+".to_string(), "whitespace".to_string()],
            vec![r"^//[^\r\n]*".to_string(), "comment".to_string()],
            // Digits
            vec![
                r"^(\d*\.\d+)|^(\d+\.\d*|\d+)".to_string(),
                "number".to_string(),
            ],
            // Operators
            vec![r"^\+".to_string(), "+".to_string()],
            vec![r"^\-".to_string(), "-".to_string()],
            vec![r"^\/".to_string(), "/".to_string()],
            vec![r"^\*".to_string(), "*".to_string()],
            // Parenthesis
            vec![r"^\(".to_string(), "(".to_string()],
            vec![r"^\)".to_string(), ")".to_string()],
            // Error
            vec![r"^.".to_string(), "error".to_string()],
        ]
    })
    .to_vec();

    return patterns;
}

pub fn tokenize(expression: String) -> Vec<HashMap<String, String>> {
    let line = 1;
    let mut column = 1;
    let mut tokens: Vec<HashMap<String, String>> = Vec::new();

    let mut ex: String = expression.clone();

    loop {
        // Makes empty dictionary / hashmap
        let mut dict = HashMap::new();
        let patterns = get_patterns();

        // Defaults tag to None
        dict.insert("tag".to_string(), "None".to_string());

        // Iterates through all patterns
        for pattern in patterns {
            let regex = pattern[0].clone();
            let tag = pattern[1].clone();

            let re = Regex::new(&regex, Flags::default()).unwrap();

            // Checks if regex matches the character
            if re.is_match(&ex.to_string()) {
                dict.insert("tag".to_string(), tag.to_string());

                // Pulls out the number
                if let Some(m) = re.find(&ex.to_string()) {
                    // Pulls the value from the number
                    let val = &ex.to_string()[m.start..m.end];

                    // Sets value
                    if tag == "number" {
                        dict.insert("value".to_string(), val.to_string());
                    } else {
                        dict.insert("value".to_string(), "null".to_string());
                    }

                    ex = ex[m.end..].to_string();
                }
                break;
            }
        }

        // Skips whitespace and comments
        if dict["tag"] == "whitespace".to_string() || dict["tag"] == "comment".to_string() {
            continue;
        }

        // Inserts the line and column counts
        dict.insert("line".to_string(), line.to_string());
        dict.insert("column".to_string(), column.to_string());

        // If default is still present, we reached the end
        if dict["tag"] == "None".to_string() {
            dict.insert("value".to_string(), "null".to_string());
            tokens.push(dict.clone());
            break;
        }

        tokens.push(dict.clone());

        column += 1;
    }

    if TEST {
        for i in 0..tokens.len() {
            println!("val {} with tag {}", tokens[i]["value"], tokens[i]["tag"]);
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests;
