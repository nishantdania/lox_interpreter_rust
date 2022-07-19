use crate::token_type::TokenType;
use crate::literal::Literal;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line_number: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("type", &self.token_type)
            .field("lexeme", &self.lexeme)
            .field("literal", &self.literal)
            .finish()
    }
}
