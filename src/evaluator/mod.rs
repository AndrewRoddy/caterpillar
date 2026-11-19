use crate::ast::Node;

    pub fn evaluate(ast: Node) -> f32 {
    // Gets the tags value
    if let Node::Fork { op, left, right } = ast.clone() {
        let tag = op.to_string();
        if tag == "+" {
            return evaluate(*left) + evaluate(*right);
        } else if tag == "-" {
            return evaluate(*left) - evaluate(*right);
        } else if tag == "unary-" {
            return evaluate(*left) * -1.0;
        } else if tag == "*" {
            return evaluate(*left) * evaluate(*right);
        } else if tag == "/" {
            return evaluate(*left) / evaluate(*right);
        }
    }

    // If it is a number, return the number
    if let Node::Value(v) = ast {
        let total = v.parse::<f32>().unwrap();
        return total.try_into().unwrap();
    }
    panic!("Unknown AST node");
}

#[cfg(test)]
mod tests;
