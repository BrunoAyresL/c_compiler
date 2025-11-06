use core::fmt;

use crate::lexer::{new_lexer, Lexer};
use crate::token::{Token, Type};
use crate::node::{ParserNode};
pub struct Parser {
    lexer: Lexer,
    next_token: Token,
}

pub fn new_parser(input: &str) -> Result<Parser, ParserError> {
    let mut p = Parser {
        lexer: new_lexer(input),
        next_token: Token::Invalid,
    };
    p.next_token = match p.lexer.next_token() {
        Ok(v) => v,
        Err(_) => return Err(ParserError::InvalidInput),
    };
    Ok(p)
}


#[derive(Debug)]
pub enum ParserError {
    InvalidInput,
    InvalidToken {t: Token, msg: String},
    UnexpectedToken {expected: Token, found: Token, pos: usize},

}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::InvalidToken {t, msg} => write!(f, "ParserError: invalid token '{:?}' found at {}", t, msg),
            ParserError::InvalidInput => write!(f, "ParserError: string input is invalid"),
            ParserError::UnexpectedToken { expected, found, pos } 
                => write!(f, "ParserError: expected '{:?}', found '{:?}' at {}", expected, found, pos),
 
        }
    }
}



impl Parser {
     
    pub fn parse(&mut self) -> Result<ParserNode, ParserError> {
        self.parse_block()
    }
    fn parse_block(&mut self) -> Result<ParserNode, ParserError> {
        let mut statements: Vec<ParserNode> = Vec::new(); 
        while self.next_token != Token::CloseBracket && self.next_token != Token::EoF{
            statements.push(self.parse_stmt()?);
            
        }
        self.read_token();
        Ok(ParserNode::Block(statements))
    } 
    fn parse_stmt(&mut self) -> Result<ParserNode, ParserError> {
        match self.next_token {
            Token::If => self.parse_if(),
            Token::Return => self.parse_return(),
            Token::Type(t) => {
                self.read_token();
                if !matches!(self.next_token,Token::Ident(_)) {
                    return Err(ParserError::InvalidToken{ t:self.next_token.clone(), msg:String::from("parse_stmt > not a Ident")})
                }
                let mut ident = String::new();
                match &self.next_token {
                    Token::Ident(name) => {
                        ident.push_str(name);
                    }
                    _ => return Err(ParserError::UnexpectedToken
                        { expected: Token::Ident(String::new()), found:self.next_token.clone(), pos: self.lexer.curr})
                }

                self.read_token();
                match self.next_token {
                    Token::Assign | Token::Semicolon => self.parse_var_decl(ident, t.clone()),
                    Token::OpenParenthesis => self.parse_func_decl(ident, t.clone()),
                    _ => return Err(ParserError::UnexpectedToken
                        { expected: Token::OpenParenthesis, found:self.next_token.clone(), pos: self.lexer.curr})
                }
                
            },
            Token::OpenBracket => self.parse_block(),
            _ => {
                 let n = self.parse_expression()?;
                 if self.next_token == Token::Semicolon {
                    self.read_token();
                 }
                 Ok(n)
             } ,
        }
    }
    fn parse_if(&mut self) -> Result<ParserNode, ParserError> {
        self.read_token();
        self.expect(Token::OpenParenthesis)?;
        let cond = self.parse_logical_or()?;
        self.expect(Token::CloseParenthesis)?;
        self.expect(Token::OpenBracket)?;
        let block = self.parse_block()?;
        let mut else_stmt = Option::None;
        if self.next_token == Token::Else {
            self.read_token();
            else_stmt = Some(Box::from(self.parse_else()?));
        }
        Ok(ParserNode::If { cond: Box::from(cond), block: Box::from(block), else_stmt: else_stmt })
    }
    fn parse_else(&mut self) -> Result<ParserNode, ParserError> {
        if self.next_token != Token::If && self.next_token != Token::OpenBracket {
            return Err(ParserError::UnexpectedToken
                { expected:Token::OpenBracket, found: self.next_token.clone(), pos: self.lexer.curr});
        }
        if self.next_token == Token::OpenBracket {
            self.read_token();
            return self.parse_block();
        }
        self.parse_stmt()
    }
    fn parse_return(&mut self) -> Result<ParserNode, ParserError> {
        self.read_token();
        let exp = self.parse_logical_or()?;    
        self.expect(Token::Semicolon)?;
        Ok(ParserNode::Return { exp: Box::from(exp) })
    }
    fn parse_var_decl(&mut self, ident: String, t: Type) -> Result<ParserNode, ParserError> {
        let ident_node = Box::from(ParserNode::Var{ident, ntype: t});
        if self.next_token == Token::Assign {
            self.read_token();
            let exp = self.parse_logical_or()?;
            self.expect(Token::Semicolon)?;
            Ok(ParserNode::Declare { ident: ident_node, exp: Some(Box::from(exp)), ntype: t })
        } else {
            self.expect(Token::Semicolon)?;
            Ok(ParserNode::Declare { ident: ident_node, exp: None, ntype: t})
        }
    }
    fn parse_func_decl(&mut self, ident: String, t: Type) -> Result<ParserNode, ParserError> {
        self.read_token();
        let args = self.parse_func_args()?;
        self.expect(Token::CloseParenthesis)?;
        self.expect(Token::OpenBracket)?;
        let block = self.parse_block()?;
        let ident_node = Box::from(ParserNode::Var{ident, ntype: t});
        Ok(ParserNode::FuncDecl { ident: ident_node, args, block: Box::from(block), size: 0, ntype: t })
    }
    fn parse_func_args(&mut self) -> Result<Vec<ParserNode>, ParserError> {
        let mut args = Vec::new();
        while self.next_token != Token::CloseParenthesis {
            let mut ntype = Type::Void;
            if let Token::Type(t) = &self.next_token {
                ntype = t.clone();
                self.read_token();
            } else {
                return Err(ParserError::UnexpectedToken
                    { expected: Token::Type(Type::Void), found: self.next_token.clone(), pos: self.lexer.curr })
            }
            match &self.next_token {
                Token::Ident(name) => {
                    args.push(ParserNode::Var{ident: name.clone(), ntype}); 
                    self.read_token();
                },
                _ => return Err(ParserError::InvalidToken { t:self.next_token.clone(), msg: String::from("parse_func_args") }),
            }
            if self.next_token == Token::Comma {
                self.read_token();
            }
        }
        Ok(args)
    }

    fn parse_expression(&mut self) -> Result<ParserNode, ParserError> {
        let mut exps = vec![self.parse_assign()?];
        loop {
            match self.next_token {
                Token::Comma => {
                    self.read_token();
                    exps.push(self.parse_assign()?);
                },
                _ => break,
            }
        }
        Ok(ParserNode::Expression(exps))
    }
    fn parse_assign(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_logical_or()?;
        match self.next_token {
            Token::Assign => {
                self.read_token();
                let b = self.parse_logical_or()?;
                self.expect(Token::Semicolon)?;
                a = ParserNode::Assign { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            _ => return Ok(a),
        }
        Ok(a)    
    }
    fn parse_logical_or(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_logical_and()?;
        loop {
            match self.next_token {
                Token::LogicalOr => {
                    self.read_token();
                    let b = self.parse_logical_and()?;
                    a = ParserNode::LogicalOr { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)     
    }
    fn parse_logical_and(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_bitwise_or()?;
        loop {
            match self.next_token {
                Token::LogicalAnd => {
                    self.read_token();
                    let b = self.parse_bitwise_or()?;
                    a = ParserNode::LogicalAnd { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)   
    }
    fn parse_bitwise_or(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_bitwise_xor()?;
        loop {
            match self.next_token {
                Token::BitwiseOr => {
                    self.read_token();
                    let b = self.parse_bitwise_xor()?;
                    a = ParserNode::BitwiseOr { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)   
    }
    fn parse_bitwise_xor(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_bitwise_and()?;
        loop {
            match self.next_token {
                Token::BitwiseXor => {
                    self.read_token();
                    let b = self.parse_bitwise_and()?;
                    a = ParserNode::BitwiseXor { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)   
    }
    fn parse_bitwise_and(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_equality()?;
        loop {
            match self.next_token {
                Token::BitwiseAnd => {
                    self.read_token();
                    let b = self.parse_equality()?;
                    a = ParserNode::BitwiseAnd { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)   
    }
    fn parse_equality(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_relational()?;
        match self.next_token {
            Token::Equal => {
                self.read_token();
                let b = self.parse_relational()?;
                a = ParserNode::Equal { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::NotEqual => {
                self.read_token();
                let b = self.parse_relational()?;
                a = ParserNode::NotEqual { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            _ => return Ok(a),
        }
        Ok(a )   
    }
    fn parse_relational(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_shift()?;
        match self.next_token {
            Token::Greater => {
                self.read_token();
                let b = self.parse_shift()?;
                a = ParserNode::Greater { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::GreaterEqual => {
                self.read_token();
                let b = self.parse_shift()?;
                a = ParserNode::GreaterEqual { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::Less => {
                self.read_token();
                let b = self.parse_shift()?;
                a = ParserNode::Less { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::LessEqual => {
                self.read_token();
                let b = self.parse_shift()?;
                a = ParserNode::LessEqual { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            _ => return Ok(a),
        }
        Ok(a)    
    }    
    fn parse_shift(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_additive()?;
        loop {
            match self.next_token {
                Token::ShiftLeft => {
                    self.read_token();
                    let b = self.parse_additive()?;
                    a = ParserNode::ShiftLeft { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::ShiftRight => {
                    self.read_token();
                    let b = self.parse_additive()?;
                    a = ParserNode::ShiftRight { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)
    }
    fn parse_additive(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_term()?;
        loop {
            match self.next_token {
                Token::Plus => {
                    self.read_token();
                    let b = self.parse_term()?;
                    a = ParserNode::Add { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::Minus => {
                    self.read_token();
                    let b = self.parse_term()?;
                    a = ParserNode::Sub { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        Ok(a)
    }
    fn parse_term(&mut self) -> Result<ParserNode, ParserError> {
        let mut a = self.parse_unary()?;
        loop {
            match self.next_token {
                Token::Asterisk => {
                    self.read_token();
                    let b = self.parse_unary()?;
                    a = ParserNode::Mul { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::Divide => {
                    self.read_token();
                    let b = self.parse_unary()?;
                    a = ParserNode::Div { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::Mod => {
                    self.read_token();
                    let b = self.parse_unary()?;
                    a = ParserNode::Mod { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                }
                _ => break,
            }
        }
        Ok(a)
    }
    fn parse_unary(&mut self) -> Result<ParserNode, ParserError> {
        match self.next_token {
            Token::Not => {
                self.read_token();
                let node = self.parse_unary()?;
                Ok(ParserNode::Not { val: Box::from(node) })
            },
            Token::Tilde => {
                self.read_token();
                let node = self.parse_unary()?;
                Ok(ParserNode::Complement { val: Box::from(node) })
  
            },
            Token::Minus => {
                self.read_token();
                let node = self.parse_unary()?;
                Ok(ParserNode::Neg { val: Box::from(node) })
            },
            _ => self.parse_factor(),
        }
    }
    fn parse_factor(&mut self) -> Result<ParserNode, ParserError> {
        let token = self.next_token.clone();
        match token {
            Token::Ident(ref id) => {
                self.read_token();
                if self.next_token == Token::OpenParenthesis {
                    return self.parse_func_call(id.clone())
                }
                Ok(ParserNode::Var{ident: id.clone(), ntype: Type::Void})
            },
            Token::Const(val) => {
                let node = ParserNode::Const(val);
                self.read_token();
                Ok(node)
            },
            Token::OpenParenthesis => {
                self.read_token();
                let exp = self.parse_logical_or()?;
                self.expect(Token::CloseParenthesis)?;
                Ok(ParserNode::SubExp { val: Box::from(exp) })
            },
            _ => return Err(ParserError::InvalidToken{ t:self.next_token.clone(), msg:String::from("parse_factor")}),
        }
    }
    fn parse_func_call(&mut self, id: String) -> Result<ParserNode, ParserError> {
        self.read_token();
        let mut args = Vec::new();
        loop {
            match self.next_token {
                Token::Comma => self.read_token(),
                Token::CloseParenthesis => break,
                _ => args.push(self.parse_assign()?),   
            }
        }
        self.read_token();

        Ok(ParserNode::FuncCall { ident: id.clone(), args: args })
    }
    pub fn read_token(&mut self) {
        if self.next_token != Token::EoF {
            self.next_token = self.lexer.next_token().expect("Erro (rt)");
        }
    }
    fn expect(&mut self, t: Token) -> Result<Token, ParserError> {
        let tok = self.next_token.clone();
        if self.next_token == t {
            self.read_token();
            Ok(tok)
        } else {
            Err(ParserError::UnexpectedToken{expected: t, found: tok, pos: self.lexer.curr})
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    fn collect_nodes(input: &str) -> ParserNode {
        let mut parser = new_parser(input).unwrap();
        parser.parse().unwrap()
    }

    #[test]
    fn parser_declare() {
        let cases = [
            ("int x;", "int x;\n"),
            ("int y = 2;", "int y = 2;\n"),
            ("int z = 2*3;", "int z = (2 * 3);\n"),
        ];
        
        for (input, expected) in cases {
            let got = collect_nodes(input);
            let output = got.to_string();
            assert_eq!(expected, output, "failed at: {}", input);
        }
    }
    #[test]
    fn parser_function() { 
        let cases = [
            ("int x() {}", "int x()0 {}"),
            ("y(a, b)", "y(a,b)"),
            ("int z(int a) {}","int z(int a)0 {}"),
        ];
        
        for (input, expected) in cases {
            let got = collect_nodes(input);
            let output = got.to_string();
            assert_eq!(expected, output, "failed at: {}", input);
        }
    }
    #[test]
    fn parser_expression() { 
        let cases = [
            ("5 + 3 == 2+1>>3*4", "((5 + 3) == ((2 + 1) >> (3 * 4)))"),
            ("a + b * c - d / e & f | g ^ h << 2 >> 1 && i || j", "((((((a + (b * c)) - (d / e)) & f) | (g ^ ((h << 2) >> 1))) && i) || j)"),
            ("a + b * c", "(a + (b * c))"),
            ("a - b / c % d", "(a - ((b / c) % d))"),
            ("a & b | c ^ d << e", "((a & b) | (c ^ (d << e)))"),
            ("(a + b) * c", "(((a + b)) * c)"),
            ("!(a + b * ~c)", "!((a + (b * ~c)))"),
            ("!a + b * ~c", "(!a + (b * ~c))"),
        ];
        
        for (input, expected) in cases {
            let got = collect_nodes(input);
            let output = got.to_string();
            assert_eq!(expected, output, "failed at: {}", input);
            
        }
    }
}
