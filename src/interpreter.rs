use crate::parser::{Node, Operator};

pub fn visit(node: Node) -> f64 {
    match node {
        Node::Number(n) => n,
        Node::Unary { op, node } => match op {
            Operator::Plus => visit(*node),
            Operator::Minus => -visit(*node),
            _ => unreachable!(),
        },
        Node::Binary { op, left, right } => match op {
            Operator::Plus => visit(*left) + visit(*right),
            Operator::Minus => visit(*left) - visit(*right),
            Operator::Multiply => visit(*left) * visit(*right),
            Operator::Divide => visit(*left) / visit(*right),
            Operator::Modulo => visit(*left) % visit(*right),
        },
    }
}
