use super::*;

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
