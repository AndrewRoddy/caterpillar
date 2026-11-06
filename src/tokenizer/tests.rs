use super::*;

#[test]
fn test_digits() {
    let token = tokenize("123".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], "123".to_string());
    assert_eq!(token[1]["tag"], "None".to_string());

    let token = tokenize("1".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], "1".to_string());
    assert_eq!(token[1]["tag"], "None".to_string());
}

#[test]
fn test_operators() {
    let tokens = tokenize("+ - * / ( )".to_string());
    let tags = vec!["+", "-", "*", "/", "(", ")", "None"];

    for i in 0..tags.len() {
        assert_eq!(tokens[i]["tag"], tags[i]);
    }
}
