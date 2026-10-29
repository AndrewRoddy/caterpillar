use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Clone)]
enum Node {
    Value(String),
    Fork {
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },
}

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

fn print_node(item: Node) {
    if let Node::Fork { op, left, right } = item {
        println!("{}", op);
        print_node(*left);
        print_node(*right);
    } else if let Node::Value(v) = item {
        println!("{}", v);
    }
}

// Done unless I add (
fn parse_factor(tokens: Vec<HashMap<String, String>>) -> (Node, Vec<HashMap<String, String>>) {
    // Gets the first token
    let mut token: HashMap<String, String> = tokens[0].clone();
    print!("parse_factor token -> ");
    print_token(token.clone());

    // If the token is a number
    if token["tag"] == "number" {
        let node = Node::Value(token["value"].to_string());

        // Makes copy without first element
        let mut tokens_copy = tokens.clone();
        tokens_copy.remove(0);
        return (node, tokens_copy);
    }

    // I need to add ( at some point here
    panic!("Got to the end of parse_factor");
}

// Done (unless I add -1)
fn parse_unary(tokens: Vec<HashMap<String, String>>) -> (Node, Vec<HashMap<String, String>>) {
    let (mut node, mut tokens_temp) = parse_factor(tokens);
    // Done (unless I add -1)
    return (node, tokens_temp);
}

fn parse_term(tokens: Vec<HashMap<String, String>>) -> (Node, Vec<HashMap<String, String>>) {
    // Makes dictionaries
    let (mut left, mut tokens_copy) = parse_unary(tokens.clone());
    println!("Printing left at start of parse_term");
    print_node(left.clone());

    loop {
        // Gets the first token
        let mut token: HashMap<String, String> = tokens_copy[0].clone();
        print!("parse_term token -> ");
        print_token(token.clone());

        // while token == *
        if token["tag"] != "*" {
            break;
        }

        tokens_copy.remove(0); // Removes first element from tokens
        let (mut right_temp, tokens_copy) = parse_unary(tokens_copy.clone());

        left = Node::Fork {
            op: token["tag"].clone(),
            left: Box::new(left.clone()),
            right: Box::new(right_temp.clone()),
        };

        // Prints left
        println!("Printing left in loop in parse_term");
        print_node(left.clone());

        // Prints right
        println!("Printing right in loop in parse_term");
        print_node(right_temp.clone());
    }

    return (left, tokens_copy);
}

fn parse_expression(tokens: Vec<HashMap<String, String>>) -> (Node, Vec<HashMap<String, String>>) {
    // Prints tokens
    println!("Start of parse_expression");
    print_tokens(tokens.clone());

    let (mut left, mut tokens_copy) = parse_term(tokens.clone());

    println!("Printing left in parse_expression");
    print_node(left.clone());

    loop {
        let mut token = tokens_copy[0].clone();

        if token["tag"] != '+'.to_string() {
            break;
        }

        println!("In loop in parse_expression.");
        print_token(token.clone());

        tokens_copy.remove(0); // [1:] equivalent

        let (mut right, tokens_copy) = parse_term(tokens_copy.clone());

        left = Node::Fork {
            op: token["tag"].clone(),
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
        };

        // Prints left
        println!("Printing left in loop in parse_expression");
        print_node(left.clone());

        // Prints right
        println!("Printing right in loop in parse_expression");
        print_node(right.clone());
    }

    return (left, tokens_copy);
}

fn parse(tokens: Vec<HashMap<String, String>>) -> Node {
    let (mut ast, mut output_tokens) = parse_expression(tokens);
    println!("Printing ast before return");
    print_node(ast.clone());
    return ast;
}
