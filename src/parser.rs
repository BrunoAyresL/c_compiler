use crate::lexer::{new_lexer, Lexer};
use crate::token::Token;
use crate::node::ParserNode;
pub struct Parser {
    lexer: Lexer,
    next_token: Token,
}

pub fn new_parser(input: &str) -> Parser {
    let mut p = Parser {
        lexer: new_lexer(input),
        next_token: Token::Invalid,
    };
    p.next_token = p.lexer.next_token().expect("Erro (p)");
    p
}

impl Parser {
     
    pub fn parse(&mut self) -> ParserNode {
        self.parse_stmt()
    }
    fn parse_block(&mut self) -> ParserNode {
        let mut statements: Vec<ParserNode> = Vec::new(); 
        while self.next_token != Token::CloseBracket && self.next_token != Token::EoF {
            statements.push(self.parse_stmt());
        }
        self.read_token();
        ParserNode::Block(statements)
    } 
    fn parse_stmt(&mut self) -> ParserNode {
        match self.next_token {
            Token::If => {
                self.read_token();
                if self.next_token != Token::OpenParenthesis {
                    return self.err("STMT: if => missing (");
                }
                self.read_token();
                let cond = self.parse_logical_or();
                if self.next_token != Token::CloseParenthesis {
                    return self.err("STMT: if => missing )");
                }
                self.read_token();
                if self.next_token != Token::OpenBracket {
                    return self.err("STMT: if => missing {");
                }
                self.read_token();
                let block = self.parse_block();
                ParserNode::If { cond: Box::from(cond), block: Box::from(block) }
            },
            Token::IntType => {
                self.read_token();
                let id = self.parse_logical_or();
                match self.next_token {
                    Token::Assign => {
                        self.read_token();
                        let exp = self.parse_logical_or();
                        if self.next_token != Token::Semicolon {
                            return self.err("STMT: var declaration => missing ;");
                        }
                        self.read_token();
                        ParserNode::Declare { ident: Box::from(id), exp: Some(Box::from(exp)) }
                    },
                    Token::OpenParenthesis => {
                        self.read_token();
                        if self.next_token != Token::CloseParenthesis {
                            return self.err("STMT: function declaration => missing )");
                        }
                        self.read_token();
                        if self.next_token != Token::OpenBracket {
                            return self.err("STMT: function declaration => missing {");
                        }
                        self.read_token();
                        let block = self.parse_block();

                        ParserNode::FuncDecl { ident: Box::from(id), block: Box::from(block) }
                    },
                    Token::Semicolon => {
                        self.read_token();
                        ParserNode::Declare { ident: Box::from(id), exp: None }
                    },
                    _ => self.err("STMT: IntType ? => invalid syntax"),
                }
                
            },
            Token::Return => {
                self.read_token();
                let exp = self.parse_logical_or();
                
                if self.next_token != Token::Semicolon {
                    return self.err("STMT: return => missing ;");
                }
                self.read_token();
                ParserNode::Return { exp: Box::from(exp) }
            },
            Token::Ident(_) => {
                self.parse_assign()
            },
            Token::Int(_) => {
                self.parse_assign()
            },

            _ => self.err("STMT: ? => invalid syntax"),
        }
    }
    fn parse_assign(&mut self) -> ParserNode {
        let mut a = self.parse_logical_or();
        match self.next_token {
            Token::Assign => {
                self.read_token();
                let b = self.parse_logical_or();
                if self.next_token != Token::Semicolon {
                    return self.err("STMT: Assign => missing ;");
                }
                self.read_token();
                a = ParserNode::Assign { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            _ => return a,
        }
        a    
    }
    fn parse_logical_or(&mut self) -> ParserNode {
        let mut a = self.parse_logical_and();
        loop {
            match self.next_token {
                Token::LogicalOr => {
                    self.read_token();
                    let b = self.parse_logical_and();
                    a = ParserNode::LogicalOr { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a     
    }
    fn parse_logical_and(&mut self) -> ParserNode {
        let mut a = self.parse_bitwise_or();
        loop {
            match self.next_token {
                Token::LogicalAnd => {
                    self.read_token();
                    let b = self.parse_bitwise_or();
                    a = ParserNode::LogicalAnd { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a   
    }
    fn parse_bitwise_or(&mut self) -> ParserNode {
        let mut a = self.parse_bitwise_xor();
        loop {
            match self.next_token {
                Token::BitwiseOr => {
                    self.read_token();
                    let b = self.parse_bitwise_xor();
                    a = ParserNode::BitwiseOr { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a   
    }
    fn parse_bitwise_xor(&mut self) -> ParserNode {
        let mut a = self.parse_bitwise_and();
        loop {
            match self.next_token {
                Token::BitwiseXor => {
                    self.read_token();
                    let b = self.parse_bitwise_and();
                    a = ParserNode::BitwiseXor { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a   
    }
    fn parse_bitwise_and(&mut self) -> ParserNode {
        let mut a = self.parse_equality();
        loop {
            match self.next_token {
                Token::BitwiseAnd => {
                    self.read_token();
                    let b = self.parse_equality();
                    a = ParserNode::BitwiseAnd { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a   
    }
    fn parse_equality(&mut self) -> ParserNode {
        let mut a = self.parse_relational();
        match self.next_token {
            Token::Equal => {
                self.read_token();
                let b = self.parse_relational();
                a = ParserNode::Equal { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::NotEqual => {
                self.read_token();
                let b = self.parse_relational();
                a = ParserNode::NotEqual { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            _ => return a,
        }
        a    
    }
    fn parse_relational(&mut self) -> ParserNode {
        let mut a = self.parse_exp();
        match self.next_token {
            Token::Greater => {
                self.read_token();
                let b = self.parse_exp();
                a = ParserNode::Greater { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::GreaterEqual => {
                self.read_token();
                let b = self.parse_exp();
                a = ParserNode::GreaterEqual { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::Less => {
                self.read_token();
                let b = self.parse_exp();
                a = ParserNode::Less { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            Token::LessEqual => {
                self.read_token();
                let b = self.parse_exp();
                a = ParserNode::LessEqual { 
                    left: Box::from(a), right: Box::from(b), 
                };
            },
            _ => return a,
        }
        a    
    }    
    fn parse_exp(&mut self) -> ParserNode {
        let mut a = self.parse_additive();
        loop {
            match self.next_token {
                Token::ShiftLeft => {
                    self.read_token();
                    let b = self.parse_additive();
                    a = ParserNode::ShiftLeft { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::ShiftRight => {
                    self.read_token();
                    let b = self.parse_additive();
                    a = ParserNode::ShiftRight { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a
    }
    fn parse_additive(&mut self) -> ParserNode {
        let mut a = self.parse_term();
        loop {
            match self.next_token {
                Token::Plus => {
                    self.read_token();
                    let b = self.parse_term();
                    a = ParserNode::Add { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::Minus => {
                    self.read_token();
                    let b = self.parse_term();
                    a = ParserNode::Sub { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                _ => break,
            }
        }
        a
    }
    fn parse_term(&mut self) -> ParserNode {
        let mut a = self.parse_unary();
        loop {
            match self.next_token {
                Token::Asterisk => {
                    self.read_token();
                    let b = self.parse_unary();
                    a = ParserNode::Mul { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::Divide => {
                    self.read_token();
                    let b = self.parse_unary();
                    a = ParserNode::Div { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                },
                Token::Mod => {
                    self.read_token();
                    let b = self.parse_unary();
                    a = ParserNode::Mod { 
                        left: Box::from(a), right: Box::from(b), 
                    };
                }
                _ => break,
            }
        }
        a
    }
    fn parse_unary(&mut self) -> ParserNode {
        match self.next_token {
            Token::Not => {
                self.read_token();
                let node = self.parse_unary();
                ParserNode::Not { val: Box::from(node) }
            },
            Token::Tilde => {
                self.read_token();
                let node = self.parse_unary();
                ParserNode::Complement { val: Box::from(node) } 
  
            },
            Token::Minus => {
                self.read_token();
                let node = self.parse_unary();
                ParserNode::Neg { val: Box::from(node) }
            },
            _ => self.parse_factor(),
        }
    }
    fn parse_factor(&mut self) -> ParserNode {
        match self.next_token {
            Token::Ident(ref id) => {
                let node = ParserNode::Var(id.clone());
                self.read_token();
                node
            },
            Token::Int(num) => {
                let node = ParserNode::Const(num);
                self.read_token();
                node
            },
            Token::OpenParenthesis => {
                self.read_token();
                let exp = self.parse_exp();
                if self.next_token != Token::CloseParenthesis {
                   self.err("FACTOR: missing )");
                }
                self.read_token();
                ParserNode::SubExp { val: Box::from(exp) }
            },
            _ => self.err("FACTOR: ? => invalid syntax"),
        }
    }
    pub fn read_token(&mut self) {
        if self.next_token != Token::EoF {
            self.next_token = self.lexer.next_token().expect("Erro (rt)");
        } 
        //print!(" {:?}", self.next_token);
    }

    fn err(&self, msg: &str) -> ParserNode {
        ParserNode::Invalid(format!("{} at {:?}", msg, self.next_token))
    }

}