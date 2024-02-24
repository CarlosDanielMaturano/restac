#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LITERAL(String),
    PLUS,
    MINUS,
    DIVIDE,
    MUTIPLY,
    LPAREN,
    RPAREN,
}
