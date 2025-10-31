use std::{sync::atomic::{AtomicUsize, Ordering}};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
#[derive(Debug)]
pub enum ParserNode {
    // block
    Block(Vec<ParserNode>),
    
    // statement (declaration)
    FuncDecl {ident: Box<ParserNode>, args: Vec<ParserNode>, block: Box<ParserNode>},
    Declare {ident: Box<ParserNode>, exp: Option<Box<ParserNode>>},

    // statement
    Assign {left: Box<ParserNode>, right: Box<ParserNode>},
    If {cond: Box<ParserNode>, block: Box<ParserNode>},
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
    Var(String),
    Const(usize),
    SubExp {val: Box<ParserNode>},

    // other
    Invalid(String),

    // block > statement > condition > > and > bit or > bit xor > bit and > equality > relational > shift > additive > term > unary > factor
}

impl ParserNode {

    pub fn gen_assembly(&self) -> String {
        let s = match self {

            ParserNode::FuncDecl { ident, args, block } => {
                format!(
                ".globl {0}
        {0}:
                {1}\n", ident.gen_assembly(), block.gen_assembly())
            },
            ParserNode::Block(statements) => {
                let mut a = String::new();
                for stmt in statements {
                    a.push_str(&stmt.gen_assembly());
                }
                a
            },
            ParserNode::Return { exp } => {
                format!(
                "{0}
                ret", exp.gen_assembly())
            },

            // logical 
            ParserNode::LogicalOr { left, right } => {
                let first_label = self.new_label();
                let end_label = self.new_label();
                format!(
                "{0}
                cmp $0, %rax
                je {first_label}
                mov $1, %rax
                jmp {end_label}
            {first_label}:
                {1}
                cmp $0, %rax
                mov $0, %rax
                setne %al
            {end_label}:", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::LogicalAnd { left, right } => {
                let first_label = self.new_label();
                let end_label = self.new_label();
                format!(
                "{0}
                cmp $0, %rax
                jne {first_label}
                jmp {end_label}
            {first_label}:
                {1}
                cmp $0, %rax
                mov $0, %rax
                setne %al
            {end_label}:", left.gen_assembly(), right.gen_assembly())
            }

            // bitwise
            ParserNode::BitwiseOr { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                or %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::BitwiseXor { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                xor %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::BitwiseAnd { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                and %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }

            // equality
            ParserNode::Equal { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                cmp %rax, %rcx
                mov $0, %rax
                sete %al", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::NotEqual { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                cmp %rax, %rcx
                mov $0, %rax
                setne %al", left.gen_assembly(), right.gen_assembly())
            }

            // relational
            ParserNode::Greater { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                cmp %rax, %rcx
                mov $0, %rax
                setg %al", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::GreaterEqual { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                cmp %rax, %rcx
                mov $0, %rax
                setge %al", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::Less { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                cmp %rax, %rcx
                mov $0, %rax
                setl %al", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::LessEqual { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                cmp %rax, %rcx
                mov $0, %rax
                setle %al", left.gen_assembly(), right.gen_assembly())
            }

            // expression
            ParserNode::ShiftLeft { left, right } => {
                format!(
                "{1}
                push %rax
                {0}
                pop %rcx
                shl %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::ShiftRight { left, right } => {
                format!(
                "{1}
                push %rax
                {0}
                pop %rcx
                shr %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }

            // additive
            ParserNode::Add { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                add %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }
            ParserNode::Sub { left, right } => {
                format!(
                "{1}
                push %rax
                {0}
                pop %rcx
                sub %rcx, %rax", left.gen_assembly(), right.gen_assembly())
            }

            // term
            ParserNode::Mul { left, right } => {
                format!(
                "{0}
                push %rax
                {1}
                pop %rcx
                imul %rcx, %rax", left.gen_assembly(), right.gen_assembly())  
            }
            ParserNode::Div { left, right } => {
                format!(
                "{1}
                push %rax
                {0}
                cqto
                pop %rcx
                idivq %rcx", left.gen_assembly(), right.gen_assembly())   
            }
            ParserNode::Mod { left, right } => {
                format!(
                "{1}
                push %rax
                {0}
                cqto
                pop %rcx
                idivq %rcx
                mov %rdx, %rax", left.gen_assembly(), right.gen_assembly())
            }

            // unary
            ParserNode::Neg { val } => {
                format!(
                "{0}
                neg %eax", val.gen_assembly())
            }
            ParserNode::Complement { val } => {
                format!(
                "{0}
                not %eax", val.gen_assembly())
            }
            ParserNode::Not { val } => {
                format!(
                "{0}
                cmpl $0, %eax
                movl $0, %eax
                sete %al", val.gen_assembly())
            }

            // factor
            ParserNode::Var(id) => {
                format!(
                "{0}", id)
            },
            ParserNode::Const(c) => {
                format!(
                "movl ${0}, %eax", c)
            },
            
            _ => String::new()
        };

        s
    }

    fn new_label(&self) -> String {
        COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("_pos{}", COUNTER.load(Ordering::Relaxed))
    }

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
            ParserNode::FuncDecl { ident, args, block } => {
                let mut s = format!("int {}(",ident.to_string());
                for arg in args {
                    s.push_str("int ");
                    s.push_str(&arg.to_string());
                    s.push_str(", ");
                }
                if args.len() != 0 { s.pop(); s.pop();}
                s.push_str(format!(") {{{}}}", block.to_string()).as_str());
                s
                
            }
            ParserNode::Declare{ ident, exp } => {
                match exp {
                    None => {
                        format!("int {};\n",ident.to_string())
                    },
                    Some(exp) => {
                        format!("int {} = {};\n",ident.to_string(), exp.to_string())
                    }
                }
                
            }

            ParserNode::Assign { left, right } => {
                format!("{} = {};\n",left.to_string(), right.to_string())
            }

            ParserNode::If { cond, block } => {
                format!("if ({}) {{\n    {}}}\n", cond.to_string(), block.to_string())
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
            ParserNode::Var(id) => {
                id.to_string()
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