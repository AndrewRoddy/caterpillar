use std::fs;
mod ast;
mod evaluator;
mod parser;
mod tokenizer;

fn main() {
    // Reads data from file
    let expression = fs::read_to_string("src/example.v").expect("Failed to read file");

    let tokens = tokenizer::tokenize(expression);
    let ast = parser::parse(tokens);
    let result = evaluator::evaluate(ast);
    println!("Result: {}", result);
}
