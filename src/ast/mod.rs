#[derive(Clone)]
pub enum Node {
    Value(String),
    Fork {
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },
}
