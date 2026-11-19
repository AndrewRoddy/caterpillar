use super::*;

#[test]
fn test_digits() {
    // Tests multiple numbers
    let token = tokenize("123".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], "123".to_string());
    assert_eq!(token[1]["tag"], "None".to_string());

    // Tests a single number
    let token = tokenize("1".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], "1".to_string());
    assert_eq!(token[1]["tag"], "None".to_string());
}

#[test] 
fn test_floats() {
    // Checks decimal in the middle
    let token = tokenize("12.5".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], "12.5".to_string());
    assert_eq!(token[1]["tag"], "None".to_string());
    
    // Check leading decimal point
    let token = tokenize(".5".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], ".5".to_string());
    assert_eq!(token[1]["tag"], "None".to_string());
    
    // Check trailing decimal point
    let token = tokenize("5.".to_string());
    assert_eq!(token[0]["tag"], "number".to_string());
    assert_eq!(token[0]["value"], "5.".to_string());
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

#[test]
fn test_expressions() {
    let tokens = tokenize("1+222*3".to_string());
    
    assert_eq!(tokens[0]["tag"], "number".to_string());
    assert_eq!(tokens[0]["value"], "1".to_string());

    assert_eq!(tokens[1]["tag"], "+".to_string());
    
    assert_eq!(tokens[2]["tag"], "number".to_string());
    assert_eq!(tokens[2]["value"], "222".to_string());

    assert_eq!(tokens[3]["tag"], "*".to_string());
    assert_eq!(tokens[4]["tag"], "number".to_string());
    assert_eq!(tokens[4]["value"], "3");

    // Tests the end is properly set to None
    assert_eq!(tokens[5]["tag"], "None".to_string());
}
