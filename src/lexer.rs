use std::io::Error;
use crate::token::Token;

pub struct Lexer {
    input: String,
    curr: usize,
    ch: u8,

}

pub fn new_lexer(input: &str) -> Lexer {
    Lexer {
        input: String::from(input),
        curr: 0,
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
        if self.curr >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.curr];
        }
    }

    fn read_int(&mut self) -> usize {
        let start = self.curr;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let num_str = self.input[start..self.curr].to_string();
        num_str.parse().unwrap()
    }

    fn read_ident(&mut self) -> String {
        let start = self.curr;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        self.input[start..self.curr].to_string()
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }

        let tok = match self.ch {
            b'{' => Token::OpenBracket,
            b'}' => Token::CloseBracket,
            b'(' => Token::OpenParenthesis,
            b')' => Token::CloseParenthesis,
            b';' => Token::Semicolon,
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

            // keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "int" => Token::IntType,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    _ => Token::Ident(ident),
                });
            },
            
            // const
            b'0'..=b'9' => return Ok(Token::Int(self.read_int())),
            0 => Token::EoF,
            _ => Token::Invalid
        };

        self.read_char();
        return Ok(tok)
    }

}

