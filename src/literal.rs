#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
    None,
}
