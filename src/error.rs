use std::fmt;

pub struct LoxError {
    pub line_number: u32,
    pub reason: String,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at line {}: {}", &self.line_number, &self.reason)
    }
}
