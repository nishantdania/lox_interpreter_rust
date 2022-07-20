use crate::expr::Expr;
use crate::literal::Literal;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, expression: &Expr) {
        let value = self.evaluate(expression);
        println!("{:?}", value);
    }

    fn evaluate(&self, expr: &Expr) -> Literal {
        println!("{:?}", expr);
        match expr {
            Expr::Literal { value } => value.clone(),
            _ => panic!("UNHANDLED")
        }
    }
}
