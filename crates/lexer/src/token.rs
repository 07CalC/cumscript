#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Fn,
    Return,
    If,
    Else,
    True,
    False,
    Null,
    Identifier(String),
    Number(f64),
    String(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Equal,
    NotEqual,
    DoubleEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Dot,
    Repeat,
    Until,
    For,
    DotDot,
    In,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Token { kind, line }
    }
}
