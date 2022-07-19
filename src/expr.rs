use crate::literal::Literal;
use crate::token::Token;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Rc<Expr>,
        operator: Token,
        right: Rc<Expr>,
    },
    Unary {
        right: Rc<Expr>,
        operator: Token,
    },
    Literal {
        value: Literal,
    },
    Grouping {
        expr: Rc<Expr>,
    },
}
