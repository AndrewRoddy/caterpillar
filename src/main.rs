use std::collections::HashMap;
use std::fs;

const DEV: bool = true;

fn main() {
    // Reads data from file
    let expression = fs::read_to_string("src/example.v").expect("Failed to read file");

    let tokens = tokenize(expression);
    let ast = parse(tokens);
}

// I think I might be going about this wrong. My first goal should be to just implement addition
// and subtraction maybe even without the regex, then slowly move to needing the regex. This will
// allow me to truly understand and learn rust without having to deal with basically needing to ask
// the chatbot.

fn tokenize(expression: String) -> Vec<HashMap<String, String>> {
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

        column += 1;
    }

    return tokens;
}

fn parse_factor(
    tokens: Vec<HashMap<String, String>>,
) -> (HashMap<String, String>, Vec<HashMap<String, String>>) {
    let mut node = HashMap::new();
    let mut token: HashMap<String, String> = tokens[0].clone();
    if token["tag"] == "number" {
        node.insert("tag".to_string(), "number".to_string());
        node.insert("value".to_string(), token["value"].clone());

        // Removes first element of tokens
        let mut tokens_temp: Vec<HashMap<String, String>> = tokens.clone();
        tokens_temp.remove(0);

        return (node, tokens_temp);
    }
    return (node, tokens);
}

fn parse_unary(
    tokens: Vec<HashMap<String, String>>,
) -> (HashMap<String, String>, Vec<HashMap<String, String>>) {
    let mut node = HashMap::new();
    let mut tokens_temp: Vec<HashMap<String, String>> = Vec::new();
    (node, tokens_temp) = parse_factor(tokens);

    return (node, tokens_temp);
}

fn parse_term(
    tokens: Vec<HashMap<String, String>>,
) -> (HashMap<String, String>, Vec<HashMap<String, String>>) {
    let mut left = HashMap::new();
    return (left, tokens);
}

fn parse_expression(
    tokens: Vec<HashMap<String, String>>,
) -> (Vec<HashMap<String, String>>, Vec<HashMap<String, String>>) {
    let mut left = HashMap::new();
    let mut op: HashMap<String, String> = HashMap::new();
    let mut right = HashMap::new();

    // Makes another copy of tokens
    let mut tokens_copy = tokens.clone();

    (left, tokens_copy) = parse_term(tokens_copy);
    while tokens_copy[0]["tag"] == "+".to_string() || tokens_copy[0]["tag"] == "-".to_string() {
        let op_str = tokens_copy[0]["tag"].to_string();
        println!("OP_STR: {}", op_str);

        // Makes a copy of tokens
        let mut tokens_temp: Vec<HashMap<String, String>> = tokens_copy.clone();
        tokens_temp.remove(0);

        (right, tokens_copy) = parse_term(tokens_temp);

        // Makes op its own vector to keep all of the types the same
        // let mut op_vec: Vec<HashMap<String, String>> = Vec::new();
        // let mut op_dict: HashMap<String, String> = HashMap::new();
        // op_dict.insert("tag".to_string(), op.to_string());
        // op_vec.push(op_dict);

        // Adds values of tag, left, and right to new hashmap
        // let mut dict = HashMap::new();
        // dict.insert("tag".to_string(), op_vec);
        // dict.insert("left".to_string(), left);
        // dict.insert("right".to_string(), right);
        //
        // left.clear(); // Clears out left before pushing in the new dictionary
        // left.push(dict);
    }
    return (tokens_copy.clone(), tokens_copy);
}

fn parse(tokens: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    let mut ast: Vec<HashMap<String, String>> = Vec::new();
    let mut bst: Vec<HashMap<String, String>> = Vec::new();
    (ast, bst) = parse_expression(tokens);
    return ast;
}
