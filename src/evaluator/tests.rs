use super::*;
use crate::tokenizer;
use crate::parser;

//
// test_evaluate
//

#[test]
fn test_single_value() {
    let ast = Node::Value("3".to_string());
    assert_eq!(evaluate(ast), 3 as f32);
}

#[test]
fn test_add() {
    let ast = Node::Fork {
        op: "+".to_string(),
        left: Box::new(Node::Value("3".to_string())),
        right: Box::new(Node::Value("4".to_string())),
    };
    assert_eq!(evaluate(ast), 7 as f32);
}

#[test]
fn test_multiply() {
    let ast = Node::Fork {
        op: "*".to_string(),
        left: Box::new(Node::Fork {
            op: "+".to_string(),
            left: Box::new(Node::Value("3".to_string())),
            right: Box::new(Node::Value("4".to_string())),
        }),
        right: Box::new(Node::Value("5".to_string())),
    };
    assert_eq!(evaluate(ast), 35 as f32);
}

#[test]
fn test_parenthesis() {
    // Tests parenthesis
    let tokens = tokenizer::tokenize("3*(4+5)".to_string());
    let (ast, _) = parser::parse_expression(tokens);
    assert_eq!(evaluate(ast), 27 as f32);
}
    
#[test]
fn test_unary_negative() {
    // Tests unary negative
    let tokens = tokenizer::tokenize("-1.5+2".to_string());
    let (ast, _) = parser::parse_expression(tokens);
    assert_eq!(evaluate(ast), 0.5);
}

#[test]
fn test_multiplied_unary() {
    let tokens = tokenizer::tokenize("3*-2".to_string());
    let (ast, _) = parser::parse_expression(tokens);
    assert_eq!(evaluate(ast), -6 as f32);
}    

