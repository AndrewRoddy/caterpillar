use crate::ast::Node;
use std::collections::HashMap;

pub fn parse(tokens: Vec<HashMap<String, String>>) -> Node {
    let (ast, _output_tokens) = parse_expression(tokens);
    println!("Printing ast before return");
    print_node(ast.clone());
    return ast;
}

// Done unless I add (
fn parse_factor(tokens: Vec<HashMap<String, String>>) -> (Node, Vec<HashMap<String, String>>) {
    // Gets the first token
    let token: HashMap<String, String> = tokens[0].clone();
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
    let (node, tokens_temp) = parse_factor(tokens);
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
        let token: HashMap<String, String> = tokens_copy[0].clone();
        print!("parse_term token -> ");
        print_token(token.clone());

        // while token == *
        // Makes sure we don't run out of tokens
        if tokens_copy.is_empty() || token["tag"] != "*" {
            break;
        }

        tokens_copy.remove(0); // Removes first element from tokens
        let right_temp: Node;
        (right_temp, tokens_copy) = parse_unary(tokens_copy.clone());

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
        let token = tokens_copy[0].clone();

        // Makes sure we don't run out of tokens
        if tokens_copy.is_empty() || token["tag"] != '+'.to_string() {
            break;
        }

        println!("In loop in parse_expression.");
        print_token(token.clone());

        tokens_copy.remove(0); // [1:] equivalent

        let right: Node;
        (right, tokens_copy) = parse_term(tokens_copy.clone());

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

fn print_node(item: Node) {
    if let Node::Fork { op, left, right } = item {
        println!("{}", op);
        print_node(*left);
        print_node(*right);
    } else if let Node::Value(v) = item {
        println!("{}", v);
    }
}
