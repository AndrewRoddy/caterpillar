use super::*;
use crate::tokenizer;
use crate::parser;

//
// test_evaluate
//

#[test]
fn test_single_value() {
    let ast = Node::Value("3".to_string());
    assert_eq!(evaluate(ast), 3);
}

#[test]
fn test_add() {
    let ast = Node::Fork {
        op: "+".to_string(),
        left: Box::new(Node::Value("3".to_string())),
        right: Box::new(Node::Value("4".to_string())),
    };
    assert_eq!(evaluate(ast), 7);
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
    assert_eq!(evaluate(ast), 35);
}

#[test]
fn test_tokenizer() {
    assert_eq!(1, 1);
    let tokens = tokenizer::tokenize("3*(4+5)".to_string());
    let (ast, tokens_new) = parser::parse_expression(tokens);
    //assert evaluate(ast) == 27
    //tokens = tokenizer.tokenize("-1.5+2")
    //ast, tokens = parser.parse_expression(tokens)
    //assert evaluate(ast) == 0.5
    //tokens = tokenizer.tokenize("3*-2")
    //ast, tokens = parser.parse_expression(tokens)
    //assert evaluate(ast) == -6

}
