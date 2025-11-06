use crate::node::ConstValue;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]

pub enum Type {
    Int,
    Float,
    Double,
    Char,
    Void,
    None,
}
impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => format!("int"),
            Type::Float => format!("float"),
            Type::Double => format!("double"),
            Type::Char => format!("char"),
            Type::Void => format!("void"),
            Type::None => format!("--- NONE ---"),
        }
    }
}

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub enum Token {
    Type(Type),
    Ident(String),
    Const(ConstValue),
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
    OpenSquareBracket,
    CloseSquareBracket,
    Comma,
    SingleQuote,
    DoubleQuote,
    
    EoF,
    Invalid
    
}

impl Token {
    
}