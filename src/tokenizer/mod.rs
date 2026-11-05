use monster_regex::{Flags, Regex};
use std::collections::HashMap;
use std::sync::LazyLock;

const TEST: bool = true;

fn get_patterns() -> Vec<Vec<String>> {
    let patterns: Vec<Vec<String>> = LazyLock::new(|| {
        vec![
            vec![r"\d*\.\d+|\d+\.\d*|\d+".to_string(), "number".to_string()],
            vec![r"\+".to_string(), "+".to_string()],
            vec![r"\*".to_string(), "*".to_string()],
        ]
    })
    .to_vec();

    return patterns;
}

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

        let patterns = get_patterns();

        // Defaults tag to None
        dict.insert("tag".to_string(), "None".to_string());

        // Iterates through all patterns
        for pattern in patterns {
            let regex = pattern[0].clone();
            let tag = pattern[1].clone();

            let re = Regex::new(&regex, Flags::default()).unwrap();

            // Checks if regex matches the character
            if re.is_match(&x.to_string()) {
                if TEST {
                    println!("{} with tag {}", x, tag);
                }
                dict.insert("tag".to_string(), tag.to_string());
                break;
            }
        }

        // Checks the patterns (very bad)
        if x.to_digit(10) != None {
            dict.insert("value".to_string(), x.to_string());
        }

        if !dict.contains_key("value") {
            dict.insert("value".to_string(), "null".to_string());
        }

        // Inserts the line and column counts
        dict.insert("line".to_string(), line.to_string());
        dict.insert("column".to_string(), column.to_string());

        tokens.push(dict);

        column += 1;
    }

    return tokens;
}
