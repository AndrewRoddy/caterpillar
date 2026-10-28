use std::collections::HashMap;
use std::fs;
use std::io::Write;

const DEV: bool = true;

fn main() {
    // Reads data from file
    let expression = fs::read_to_string("src/example.v").expect("Failed to read file");

    let tokens = tokenize(expression);
    let ast = parse(tokens);
}

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
        if DEV {
            println!(
                "| line: {} | column: {} | tag: {} | value: {} |",
                dict["line"], dict["column"], dict["tag"], dict["value"],
            );
        }

        tokens.push(dict);

        column += 1;
    }

    return tokens;
}

fn print_tokens(tokens: Vec<HashMap<String, String>>) {
    for token in tokens {
        print_token(token);
    }
}

fn print_token(token: HashMap<String, String>) {
    let tag = token["tag"].to_string();
    let value = token["value"].to_string();

    print!("{}", tag);
    if tag == "number" {
        print!(" ({})", value);
    }
    println!(); // prints newline
}

fn print_layered(layers: u32, text: String) {
    // Builds the line before printing
    let mut line = String::new();
    for _ in 1..layers {
        line += "| ";
    }
    print!("{}{}", line, text);
}

// Done unless I add (
fn parse_factor(
    tokens: Vec<HashMap<String, String>>,
) -> (HashMap<String, String>, Vec<HashMap<String, String>>) {
    let mut node = HashMap::new();

    // Gets the first token
    let mut token: HashMap<String, String> = tokens[0].clone();
    print!("parse_factor token -> ");
    print_token(token.clone());

    if token["tag"] == "number" {
        node.insert("tag".to_string(), "number".to_string());
        node.insert("value".to_string(), token["value"].clone());

        // Removes first element of tokens
        let mut tokens_temp: Vec<HashMap<String, String>> = tokens.clone();
        tokens_temp.remove(0);

        println!("-- after removing the first token");
        print_tokens(tokens_temp.clone());

        return (node, tokens_temp);
    }

    // I need to add ( at some point here

    panic!("Got to the end of parse_factor");
}

// Done (unless I add -1)
fn parse_unary(
    tokens: Vec<HashMap<String, String>>,
) -> (HashMap<String, String>, Vec<HashMap<String, String>>) {
    let mut node = HashMap::new();
    let mut tokens_temp: Vec<HashMap<String, String>> = Vec::new();
    (node, tokens_temp) = parse_factor(tokens);
    // Done (unless I add -1)
    return (node, tokens_temp);
}

fn parse_term(
    tokens: Vec<HashMap<String, String>>,
) -> (HashMap<String, String>, Vec<HashMap<String, String>>) {
    // Makes dictionaries
    let mut left = HashMap::<String, String>::new();
    let mut op = HashMap::<String, String>::new();
    let mut right = HashMap::<String, String>::new();

    let mut tokens_copy = tokens.clone();

    (left, tokens_copy) = parse_unary(tokens_copy.clone());

    loop {
        // Gets the first token
        let mut token: HashMap<String, String> = tokens_copy[0].clone();
        print!("parse_term token -> ");
        print_token(token.clone());

        if token["tag"] != "*" {
            break;
        }

        op.insert("op".to_string(), token["tag"].to_string());

        // Removes the first element from tokens_copy
        tokens_copy.remove(0);

        (right, tokens_copy) = parse_unary(tokens_copy);

        // Creating new left
        let mut old_left: HashMap<String, String> = left.clone();
        left.clear();

        //left.insert("tag".to_string(), op);
        //left.insert("left".to_string(), old_left);
        //left.insert("right".to_string(), right);
    }

    return (left, tokens_copy);
}

fn parse_expression(
    tokens: Vec<HashMap<String, String>>,
) -> (Vec<HashMap<String, String>>, Vec<HashMap<String, String>>) {
    println!("Start of parse_expression");
    print_tokens(tokens.clone());
    // Initializes variables
    let mut left: HashMap<String, String> = HashMap::new();
    let mut op: HashMap<String, String> = HashMap::new();
    let mut right: HashMap<String, String> = HashMap::new();

    // Makes another copy of tokens
    let mut tokens_copy = tokens.clone();

    (left, tokens_copy) = parse_term(tokens_copy);

    //while tokens_copy[0]["tag"] == "+".to_string() {
    //    println!("In while in parse_expression.");
    //    print_tokens(tokens_copy.clone());
    //}
    //let op_str = tokens_copy[0]["tag"].to_string();
    //println!("OP_STR: {}", op_str);

    // Makes a copy of tokens
    //let mut tokens_temp: Vec<HashMap<String, String>> = tokens_copy.clone();
    //tokens_temp.remove(0);

    //(right, tokens_copy) = parse_term(tokens_temp);

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
    //}
    return (tokens_copy.clone(), tokens_copy);
}

fn parse(tokens: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    let mut ast: Vec<HashMap<String, String>> = Vec::new();
    let mut bst: Vec<HashMap<String, String>> = Vec::new();
    (ast, bst) = parse_expression(tokens);
    return ast;
}
