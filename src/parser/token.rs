use crate::parser::node::ConstValue;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]

pub enum Type {
    Int,
    Float,
    Double,
    Char,
    Void,
}
impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => format!("int"),
            Type::Float => format!("float"),
            Type::Double => format!("double"),
            Type::Char => format!("char"),
            Type::Void => format!("void"),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Type::Int => 4,
            Type::Float => 4,
            Type::Double => 8,
            Type::Char => 4,
            Type::Void => 4,
        }
    }
    pub fn is_number(&self) -> bool {
        *self != Type::Char
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
    For,
    While,
    
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
