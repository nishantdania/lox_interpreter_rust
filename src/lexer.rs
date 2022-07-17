use crate::error::LoxError;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    pub errors: Vec<LoxError>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF);
        self.tokens.to_vec()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),

            '!' => match self.match_next('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },

            '=' => match self.match_next('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },

            '<' => match self.match_next('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },

            '>' => match self.match_next('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },

            '/' => match self.match_next('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash),
            },

            ' ' | '\r' | '\t' => (),

            '\n' => self.line = self.line + 1,

            '"' => self.string(),

            character => {
                if character.is_digit(10) {
                    self.number();
                } else if character.is_alphabetic() {
                    self.identifier();
                } else {
                    self.errors.push(LoxError {
                        line_number: self.line,
                        reason: "LexerError: Unexpected character.".to_string(),
                    })
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("Nth character in source");
        self.current = self.current + 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Nth char in source")
            != expected
        {
            return false;
        }

        self.current = self.current + 1;

        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .expect("Nth char in source")
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("Nth char in source")
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        match token_type {
            TokenType::EOF => {
                self.tokens.push(Token {
                    lexeme: "".to_string(),
                    line_number: self.line,
                    literal,
                    token_type: TokenType::EOF,
                });
            }
            _ => {
                let lexeme = &self.source[self.start..self.current];

                self.tokens.push(Token {
                    token_type,
                    lexeme: lexeme.to_string(),
                    line_number: self.line,
                    literal,
                })
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LoxError {
                line_number: self.line - 1,
                reason: "Unterminated string.".to_string(),
            });
            return;
        }

        self.advance();

        let literal = &self.source[(self.start + 1)..(self.current - 1)];
        self.add_token_with_literal(TokenType::String, Literal::String(literal.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal = &self.source[self.start..self.current].to_string();
        let number: f32 = literal.parse().expect("String to number");
        self.add_token_with_literal(TokenType::Number, Literal::Number(number));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }
}
