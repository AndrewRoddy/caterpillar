use crate::ast::Node;

pub fn evaluate(ast: Node) -> u32 {
    // Gets the tags value
    if let Node::Fork { op, left, right } = ast.clone() {
        let tag = op.to_string();
        if tag == "+" {
            return evaluate(*left) + evaluate(*right);
        } else if tag == "*" {
            return evaluate(*left) * evaluate(*right);
        }
    }

    // If it is a number, return the number
    if let Node::Value(v) = ast {
        let total = v.parse::<i32>().unwrap();
        return total.try_into().unwrap();
    }
    panic!();
}
