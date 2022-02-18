use crate::parser::{Node, Operator};

pub fn visit(node: Node) -> Result<f64, String> {
    match node {
        Node::Number(n) => Ok(n),
        Node::Unary { op, node } => match op {
            Operator::Plus => Ok(visit(*node)?),
            Operator::Minus => Ok(-visit(*node)?),
            _ => unreachable!(),
        },
        Node::Binary { op, left, right } => match op {
            Operator::Plus => Ok(visit(*left)? + visit(*right)?),
            Operator::Minus => Ok(visit(*left)? - visit(*right)?),
            Operator::Multiply => Ok(visit(*left)? * visit(*right)?),
            Operator::Divide => {
                let right = visit(*right)?;
                if right == 0.0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(visit(*left)? / right)
                }
            }
            Operator::Modulo => {
                let right = visit(*right)?;
                if right == 0.0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(visit(*left)? % right)
                }
            }
        },
    }
}
