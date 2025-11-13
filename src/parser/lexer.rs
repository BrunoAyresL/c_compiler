use core::fmt;
use crate::{parser::node::ConstValue, parser::token::{Token, Type}};

pub struct Lexer {
    input: String,
     curr: usize,
    pub line: usize,
    pub column: usize,
    ch: u8,

}
#[derive(Debug)]
pub enum LexerError {
    InvalidChar(char, usize),
    InvalidInt(usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidChar(c, pos) => write!(f, "LexerError: invalid char '{}' found in position {}.", c, pos),
            LexerError::InvalidInt(pos) => write!(f, "LexerError: invalid int found in position {}.", pos)
        }
    }
}


pub fn new_lexer(input: &str) -> Lexer {
    Lexer {
        input: String::from(input),
        curr: 0,
        line: 1,
        column: 0,
        ch: input.as_bytes()[0],
    }
}

impl Lexer {
    
    fn peek(&self) -> u8 {
        if self.curr + 1 >= self.input.len() {
            return 0;
        } 
        return self.input.as_bytes()[self.curr + 1];
    }

    fn read_char(&mut self) {
        self.curr += 1;
        self.column += 1;
        if self.curr >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.curr];
        }
        
    }

    fn read_int(&mut self) -> Result<i32, LexerError> {
        let start = self.curr;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let num_str = self.input[start..self.curr].to_string();
        let n = num_str.parse();
        match n {
            Ok(u) => Ok(u),
            _ => return Err(LexerError::InvalidInt(self.curr))
        }
    }

    fn read_ident(&mut self) -> String {
        let start = self.curr;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        self.input[start..self.curr].to_string()
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\r' {
            if self.ch == b'\n' { 
                self.line += 1; 
                self.column = 0;
            }
            self.read_char();
        }

        let tok = match self.ch {
            b'{' => Token::OpenBracket,
            b'}' => Token::CloseBracket,
            b'(' => Token::OpenParenthesis,
            b')' => Token::CloseParenthesis,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'~' => Token::Tilde,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Divide,
            b'&' => {
                if self.peek() == b'&' {
                    self.read_char();
                    Token::LogicalAnd
                } else {
                    Token::BitwiseAnd
                }
            },
            b'|' => {
                if self.peek() == b'|' {
                    self.read_char();
                    Token::LogicalOr
                } else {
                    Token::BitwiseOr
                }
            }
            b'^' => Token::BitwiseXor,
            b'%' => Token::Mod,
            b'>' => {
                if self.peek() == b'>' {
                    self.read_char();
                    Token::ShiftRight
                } else if self.peek() == b'=' {
                    self.read_char();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            },
            b'<' => {
                if self.peek() == b'<' {
                    self.read_char();
                    Token::ShiftLeft
                } else if self.peek() == b'=' {
                    self.read_char();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            },
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            },
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            39 => {
                self.read_char();
                let t = Token::Const(ConstValue::Char(self.ch as char));
                self.read_char();
                if self.ch != 39 {
                    return Err(LexerError::InvalidChar(39 as char, self.curr))
                }
                t
            },
            // keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "int" => Token::Type(Type::Int),
                    "float" => Token::Type(Type::Float),
                    "double" => Token::Type(Type::Double),
                    "char" => Token::Type(Type::Char),
                    "void" => Token::Type(Type::Void),
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "while" => Token::While,
                    _ => Token::Ident(ident),
                });
            },
            
            // const
            b'0'..=b'9' => return Ok(Token::Const(ConstValue::Int(self.read_int()?))),
            0 => Token::EoF,
            _ => return Err(LexerError::InvalidChar(self.ch as char, self.curr)),
        };
        self.read_char();
        return Ok(tok)
    }

}

#[cfg(test)]
mod tests {
    use super::*;


    fn collect_tokens(input: &str) -> Vec<Token> {
        let mut lex = new_lexer(input);
        let mut tokens = Vec::new();
        loop {
            let t = lex.next_token().unwrap();
            tokens.push(t.clone());
            if t == Token::EoF { break; }
        }
        tokens
    }

    #[test]
    fn lexer_declare() {
        let cases = [
            ("int x;", vec![Token::Type(Type::Int), Token::Ident("x".into()), Token::Semicolon, Token::EoF]),
            ("int y = 2;", vec![Token::Type(Type::Int), Token::Ident("y".into()), Token::Assign, Token::Const(ConstValue::Int(2)), Token::Semicolon, Token::EoF]),
            ("int z = 2*3;", vec![Token::Type(Type::Int), Token::Ident("z".into()), Token::Assign, Token::Const(ConstValue::Int(2)), Token::Asterisk, Token::Const(ConstValue::Int(3)), Token::Semicolon, Token::EoF]),
        ];
        
        for (input, expected) in cases {
            let got = collect_tokens(input);
            assert_eq!(got, expected, "failed at: {}", input);
        }
    }
    #[test]
    fn lexer_function() {
        let cases = [
            ("int x() {}", vec![Token::Type(Type::Int), Token::Ident("x".into()), Token::OpenParenthesis,  Token::CloseParenthesis, Token::OpenBracket, Token::CloseBracket, Token::EoF]),
            ("y(a, b)", vec![Token::Ident("y".into()), Token::OpenParenthesis, Token::Ident("a".into()), Token::Comma, Token::Ident("b".into()), Token::CloseParenthesis, Token::EoF]),
            ("int z(int a) {}", vec![Token::Type(Type::Int), Token::Ident("z".into()), Token::OpenParenthesis, Token::Type(Type::Int), Token::Ident("a".into()), Token::CloseParenthesis, Token::OpenBracket, Token::CloseBracket, Token::EoF]),
        ];
        
        for (input, expected) in cases {
            let got = collect_tokens(input);
            assert_eq!(got, expected, "failed at: {}", input);
        }
    }
    #[test]
    fn lexer_expression() {
        let cases = [
            ("(a + b) * c", vec![Token::OpenParenthesis, Token::Ident("a".into()), Token::Plus, Token::Ident("b".into()), Token::CloseParenthesis, Token::Asterisk, Token::Ident("c".into()), Token::EoF])
        ];
        for (input, expected) in cases {
            let got = collect_tokens(input);
            assert_eq!(got, expected, "failed at: {}", input);
        }
    }

}

