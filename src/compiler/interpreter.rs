use crate::{Compile, Node, Operator};

#[derive(Debug)]
pub struct Interpreter {}

impl Compile for Interpreter {
    type Output = i32;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut result = 0i32;
        for node in ast {
            result += eval_node(&node);
        }
        result
    }
}

fn eval_node(node: &Node) -> i32 {
    match node {
        Node::Int(n) => *n,
        Node::UnaryExpr { op, child } => {
            let val = eval_node(child);
            match op {
                Operator::Plus => val,
                Operator::Minus => -val,
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let l = eval_node(lhs);
            let r = eval_node(rhs);
            match op {
                Operator::Plus => l + r,
                Operator::Minus => l - r,
            }
        }
    }
}
