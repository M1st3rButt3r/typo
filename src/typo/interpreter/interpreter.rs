use crate::parser::node::Node;

pub fn visit(node: Node) -> f32 {
    return node.visit().unwrap();
}