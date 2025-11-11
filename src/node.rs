use crate::token::Type;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ConstValue {
    Int(i32),
    Float(f32),
    Double(f64),
    Char(char),
    Void,
}
impl ConstValue {
    pub fn to_string(&self) -> String {
        match self {
            ConstValue::Int(n) => format!("{}", n),
            ConstValue::Float(n) => format!("{}", n),
            ConstValue::Double(n) => format!("{}", n),
            ConstValue::Char(c) => format!("'{}'", c), 
            ConstValue::Void => format!(""), 
        }
    }
}

#[derive(Debug)]
pub enum ParserNode {
    // block
    Block(Vec<ParserNode>),
    
    // statement (declaration)
    FuncDecl {ident: Box<ParserNode>, args: Vec<ParserNode>, block: Box<ParserNode>, ntype: Type},
    Declare {ident: Box<ParserNode>, exp: Option<Box<ParserNode>>, ntype: Type},

    // statement
    Assign {left: Box<ParserNode>, right: Box<ParserNode>},
    If {cond: Box<ParserNode>, block: Box<ParserNode>, else_stmt: Option<Box<ParserNode>> },
    Return {exp: Box<ParserNode>},

    // expreession
    Expression (Vec<ParserNode>),

    // logical OR (condition)
    LogicalOr {left: Box<ParserNode>, right: Box<ParserNode>},

    // logical AND
    LogicalAnd {left: Box<ParserNode>, right: Box<ParserNode>},

    // bitwise OR
    BitwiseOr {left: Box<ParserNode>, right: Box<ParserNode>},

    // bitwise XOR
    BitwiseXor {left: Box<ParserNode>, right: Box<ParserNode>},

    // bitwise AND
    BitwiseAnd {left: Box<ParserNode>, right: Box<ParserNode>},

    // equality
    Equal {left: Box<ParserNode>, right: Box<ParserNode>},
    NotEqual {left: Box<ParserNode>, right: Box<ParserNode>},

    // relational
    Greater {left: Box<ParserNode>, right: Box<ParserNode>},
    GreaterEqual {left: Box<ParserNode>, right: Box<ParserNode>},
    Less {left: Box<ParserNode>, right: Box<ParserNode>},
    LessEqual {left: Box<ParserNode>, right: Box<ParserNode>},
    
    // shift
    ShiftLeft {left: Box<ParserNode>, right: Box<ParserNode>},
    ShiftRight {left: Box<ParserNode>, right: Box<ParserNode>},

    // additive
    Add {left: Box<ParserNode>, right: Box<ParserNode>},
    Sub {left: Box<ParserNode>, right: Box<ParserNode>},

    // term
    Mul {left: Box<ParserNode>, right: Box<ParserNode>},
    Div {left: Box<ParserNode>, right: Box<ParserNode>},
    Mod {left: Box<ParserNode>, right: Box<ParserNode>},

    // unary
    Neg {val: Box<ParserNode>},
    Complement {val: Box<ParserNode>},
    Not {val: Box<ParserNode>},

    // factor
    FuncCall{ident: String, args: Vec<ParserNode>},
    Var{ ident: String, ntype: Type},
    Const(ConstValue),
    SubExp {val: Box<ParserNode>},

    // other
    Invalid(String),

}

impl ParserNode {

    pub fn to_string(&self) -> String {
        match self {
            // block
            ParserNode::Block(stmts) => {
                let mut s = String::new();
                for stmt in stmts {
                    s.push_str(&stmt.to_string());
                }
                s
            }

            // statement
            ParserNode::FuncDecl { ident, args, block, ntype } => {
                let mut s = format!("{} {}(", ntype.to_string(), ident.to_string());
                for arg in args {
                    s.push_str("int ");
                    s.push_str(&arg.to_string());
                    s.push_str(", ");
                }
                if args.len() != 0 { s.pop(); s.pop();}
                s.push_str(format!(") {{{}}}", block.to_string()).as_str());
                s
                
            }
            ParserNode::Declare{ ident, exp, ntype } => {
                match exp {
                    None => {
                        format!("{} {};\n", ntype.to_string(), ident.to_string())
                    },
                    Some(exp) => {
                        format!("int {} = {};\n",ident.to_string(), exp.to_string())
                    }
                }
                
            }

            ParserNode::Assign { left, right } => {
                format!("{} = {};\n",left.to_string(), right.to_string())
            }

            ParserNode::If { cond, block , else_stmt} => {
                let mut s = format!("if ({}) {{\n    {}}}", cond.to_string(), block.to_string());
                match else_stmt {
                    Some(n) => s.push_str(format!(" else {}",n.to_string()).as_str()),
                    None => s.push('\n'),
                }
                s
            }
            
            ParserNode::Return { exp } => {
                format!("return {};\n", exp.to_string())
            }

            // expression 
            ParserNode::Expression(exps) => {
                exps.into_iter().map(|exp| exp.to_string()).collect()
            }

            // logical
            ParserNode::LogicalOr { left, right } => {
                format!("({} || {})", left.to_string(), right.to_string())
            }
            ParserNode::LogicalAnd { left, right } => {
                format!("({} && {})", left.to_string(), right.to_string())
            }

            // bitwise
            ParserNode::BitwiseOr { left, right } => {
                format!("({} | {})", left.to_string(), right.to_string())
            }
            ParserNode::BitwiseXor { left, right } => {
                format!("({} ^ {})", left.to_string(), right.to_string())
            }
            ParserNode::BitwiseAnd { left, right } => {
                format!("({} & {})", left.to_string(), right.to_string())
            }

            // equality
            ParserNode::Equal { left, right } => {
                format!("({} == {})", left.to_string(), right.to_string())
            }
            ParserNode::NotEqual { left, right } => {
                format!("({} != {})", left.to_string(), right.to_string())
            }

            // relational
            ParserNode::Greater { left, right } => {
                format!("({} > {})", left.to_string(), right.to_string())
            }
            ParserNode::GreaterEqual { left, right } => {
                format!("({} >= {})", left.to_string(), right.to_string())
            }
            ParserNode::Less { left, right } => {
                format!("({} < {})", left.to_string(), right.to_string())
            }
            ParserNode::LessEqual { left, right } => {
                format!("({} <= {})", left.to_string(), right.to_string())
            }

            // expression
            ParserNode::ShiftLeft { left, right } => {
                format!("({} << {})", left.to_string(), right.to_string())
            }
            ParserNode::ShiftRight { left, right } => {
                format!("({} >> {})", left.to_string(), right.to_string())
            }

            // additive
            ParserNode::Add { left, right } => {
                format!("({} + {})", left.to_string(), right.to_string())
            }
            ParserNode::Sub { left, right } => {
                format!("({} - {})", left.to_string(), right.to_string())
            }

            // term
            ParserNode::Mul { left, right } => {
                format!("({} * {})", left.to_string(), right.to_string())
            }
            ParserNode::Div { left, right } => {
                format!("({} / {})", left.to_string(), right.to_string())
            }
            ParserNode::Mod { left, right } => {
                format!("({} % {})", left.to_string(), right.to_string())
            }

            // unary
            ParserNode::Neg { val } => {
                format!("-{}", val.to_string())
            }
            ParserNode::Complement { val } => {
                format!("~{}", val.to_string())
            }
            ParserNode::Not { val } => {
                format!("!{}", val.to_string())
            }

            // factor
            ParserNode::FuncCall { ident, args } => {
                let mut s = String::from(ident.to_string());
                s.push('(');
                for arg in args {
                    s.push_str(&arg.to_string());
                    s.push(',');
                }
                if args.len() != 0 { s.pop(); }
                s.push(')');
                s
            }
            ParserNode::Var { ident, ntype:_ } => {
                ident.to_string()
            }
            ParserNode::Const(num) => {
                    format!("{}", num.to_string())
            }
            ParserNode::SubExp { val } => {
                format!("({})", val.to_string())
            }

            // other (debug)
            ParserNode::Invalid(s) => {
                format!("\n---------- {} ----------\n", s.to_string())
            }
        }
    }
}