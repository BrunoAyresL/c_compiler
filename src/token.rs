#[derive(Debug)]
#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    IntType,
    Ident(String),
    Int(usize),
    Char(char),

    // keywords
    If,
    Else,
    Return,
    Assign,
    
    // condition 
    LogicalAnd,
    LogicalOr,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    Equal,
    NotEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // unary 
    
    Tilde,
    Not,
    
    // exp
    ShiftLeft,
    ShiftRight,

    // binary
    Mod,
    Plus,
    Minus,
    Asterisk,
    Divide,

    // separator
    Semicolon,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    
    EoF,
    Invalid
    
}

impl Token {
    
}