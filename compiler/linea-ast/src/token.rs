#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Var,
    VarUpdate,
    Display,
    For,
    From,
    Use,
    Import,
    Function,
    Return,
    If,
    Else,
    While,
    Break,
    Continue,
    TypeCast,
    True,
    False,
    Yes,
    No,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Dot,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    Arrow,
    FatArrow,
    Range,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token { token_type, line, column }
    }
}
