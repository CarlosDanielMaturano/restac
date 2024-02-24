#[derive(Debug, PartialEq)]
pub enum Token {
    LITERAL(String),
    PLUS,
    MINUS,
    DIVIDE,
    MUTIPLY,
    LPAREN,
    RPAREN,
}
