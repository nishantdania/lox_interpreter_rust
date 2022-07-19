use crate::expr::Expr;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_type::TokenType;
use std::rc::Rc;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = Rc::new(self.comparison());
            expr = Expr::Binary {
                left: Rc::new(expr),
                operator: operator,
                right,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = Rc::new(self.term());
            expr = Expr::Binary {
                left: Rc::new(expr),
                operator,
                right,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = Rc::new(self.factor());
            expr = Expr::Binary {
                left: Rc::new(expr),
                operator: operator,
                right,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = Rc::new(self.unary());
            expr = Expr::Binary {
                left: Rc::new(expr),
                operator: operator,
                right,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Rc::new(self.unary());
            return Expr::Unary {
                operator,
                right,
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::False]) {
            return Expr::Literal {
                value: Literal::Bool(false),
            };
        }
        if self.match_tokens(vec![TokenType::True]) {
            return Expr::Literal {
                value: Literal::Bool(true),
            };
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Expr::Literal {
                value: Literal::Nil,
            };
        }

        if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            match self.previous().token_type {
                TokenType::String => {
                    return Expr::Literal {
                        value: self.previous().literal.clone(),
                    }
                }
                TokenType::Number => {
                    return Expr::Literal {
                        value: self.previous().literal.clone(),
                    }
                }
                _ => (),
            }
        }

        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            );
            return Expr::Grouping {
                expr: Rc::new(expr),
            };
        } else {
            panic!("I did not see this coming!");
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.check(token_type) {
            self.advance();
        }

        panic!("Error {message}")
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("Token")
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).expect("Token")
    }
}
