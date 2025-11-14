use crate::intermediate::irgen::Operand;

// todo:
// declare
// return
// jump
// 



#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {

    Label(String),

    Goto(String),
    BeginFunc(usize),
    EndFunc,
    PushParam(Operand),
    PopParams(usize),
    
    LCall(String),

    IfZero {cond: Operand, label: String},
    Assign {dest: Operand, arg1: Operand},
    Return {dest: Operand},
    // expreession
    // Expression (Vec<ParserNode>),

    // logical OR (condition)
    LogicalOr {dest: Operand, arg1: Operand, arg2: Operand},

    // logical AND
    LogicalAnd {dest: Operand, arg1: Operand, arg2: Operand},

    // bitwise OR
    BitwiseOr {dest: Operand, arg1: Operand, arg2: Operand},

    // bitwise XOR
    BitwiseXor {dest: Operand, arg1: Operand, arg2: Operand},

    // bitwise AND
    BitwiseAnd {dest: Operand, arg1: Operand, arg2: Operand},

    // equality
    Equal {dest: Operand, arg1: Operand, arg2: Operand},
    NotEqual {dest: Operand, arg1: Operand, arg2: Operand},

    // relational
    Greater {dest: Operand, arg1: Operand, arg2: Operand},
    GreaterEqual {dest: Operand, arg1: Operand, arg2: Operand},
    Less {dest: Operand, arg1: Operand, arg2: Operand},
    LessEqual {dest: Operand, arg1: Operand, arg2: Operand},
    
    // shift
    ShiftLeft {dest: Operand, arg1: Operand, arg2: Operand},
    ShiftRight {dest: Operand, arg1: Operand, arg2: Operand},

    // addition
    Add {dest: Operand, arg1: Operand, arg2: Operand},
    Sub {dest: Operand, arg1: Operand, arg2: Operand},

    // term
    Mul {dest: Operand, arg1: Operand, arg2: Operand},
    Div {dest: Operand, arg1: Operand, arg2: Operand},
    Mod {dest: Operand, arg1: Operand, arg2: Operand},

    // unary 
    Neg {dest: Operand, arg1: Operand},
    Complement {dest: Operand, arg1: Operand},
    Not {dest: Operand, arg1: Operand},

    // assignment
    // a = 5;
    // a = b;
    // a = b << c;
    // a = 5 % c;
    // a = b + 5;
    // a = 5 - 6;
    // + - * / %    
}

impl Instruction {
    pub fn print(&self) -> String {
        match self {



            Instruction::Label(l) => {
                format!("{}:", l)
            },
            Instruction::Goto(l) => {
                format!("   Goto {}", l)
            },
            Instruction::BeginFunc(size) => {
                format!("   BeginFunc {}", size)
            },
            Instruction::EndFunc => {
                format!("   EndFunc")
            },

            Instruction::LCall(l) => {
                format!("   LCall {}", l)
            }
            Instruction::PushParam(param) => {
                format!("   PushParam {}", param.print())
            }
            Instruction::PopParams(size) => {
                format!("   PopParams {}", size)
            }

            Instruction::IfZero { cond, label } => {
                format!("   IfZero {} Goto {}", cond.print(), label)
            },

            Instruction::Return { dest } => {
                format!("   Return {}", dest.print())
            }

            Instruction::Assign { dest, arg1} => {
                format!("   {} = {}", dest.print(), arg1.print())
            },
            Instruction::LogicalOr { dest, arg1, arg2 } => {
                format!("   {} = {} || {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::LogicalAnd { dest, arg1, arg2 } => {
                format!("   {} = {} && {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::BitwiseOr { dest, arg1, arg2 } => {
                format!("   {} = {} | {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::BitwiseXor { dest, arg1, arg2 } => {
                format!("   {} = {} ^ {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::BitwiseAnd { dest, arg1, arg2 } => {
                format!("   {} = {} & {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Equal { dest, arg1, arg2 } => {
                format!("   {} = {} == {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::NotEqual { dest, arg1, arg2 } => {
                format!("   {} = {} != {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Greater { dest, arg1, arg2 } => {
                format!("   {} = {} > {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::GreaterEqual { dest, arg1, arg2 } => {
                format!("   {} = {} >= {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Less { dest, arg1, arg2 } => {
                format!("   {} = {} < {}", dest.print(), arg1.print(), arg2.print())
            },  
            Instruction::LessEqual { dest, arg1, arg2 } => {
                format!("   {} = {} <= {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::ShiftLeft { dest, arg1, arg2 } => {
                format!("   {} = {} << {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::ShiftRight { dest, arg1, arg2 } => {
                format!("   {} = {} >> {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Add { dest, arg1, arg2 } => {
                format!("   {} = {} + {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Sub { dest, arg1, arg2 } => {
                format!("   {} = {} - {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Mul { dest, arg1, arg2 } => {
                format!("   {} = {} * {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Div { dest, arg1, arg2 } => {
                format!("   {} = {} / {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Mod { dest, arg1, arg2 } => {
                format!("   {} = {} % {}", dest.print(), arg1.print(), arg2.print())
            },
            Instruction::Neg { dest, arg1} => {
                format!("   {} = -{}", dest.print(), arg1.print())
            },
            Instruction::Complement { dest, arg1} => {
                format!("   {} = ~{}", dest.print(), arg1.print())
            },
            Instruction::Not { dest, arg1} => {
                format!("   {} = !{}", dest.print(), arg1.print())
            },
            

        }
    }


    pub fn def(&self) -> Option<Operand> {
        match self {
            Instruction::Add { dest, .. } | Instruction::Sub { dest, .. } |
            Instruction::Mul { dest, .. } | Instruction::Div { dest, .. } |
            Instruction::ShiftLeft { dest, .. } | Instruction::ShiftRight { dest, .. } |
            Instruction::Mod { dest, .. } | Instruction::BitwiseAnd { dest, .. } |
            Instruction::BitwiseXor { dest, .. } | Instruction::BitwiseOr { dest, .. } |
            Instruction::LogicalAnd { dest, .. } | Instruction::LogicalOr { dest, .. } |
            Instruction::Equal { dest, .. } | Instruction::NotEqual { dest, .. } | 
            Instruction::Greater { dest, .. } | Instruction::GreaterEqual { dest, .. } |
            Instruction::Less { dest, .. } | Instruction::LessEqual { dest, .. } | 
            Instruction::Assign { dest, .. } | Instruction::Complement { dest, .. } |
            Instruction::Neg { dest, .. } | Instruction::Not { dest, .. } => {
                Some(dest.clone())
            },
            _ => None,
        }
    }

    pub fn uses(&self) -> Vec<Operand> {
        match self {
            Instruction::Add {arg1, arg2, ..} | Instruction::Sub {arg1, arg2, ..} |
            Instruction::Mul {arg1, arg2, ..} | Instruction::Div {arg1, arg2, ..} |
            Instruction::ShiftLeft {arg1, arg2, ..} | Instruction::ShiftRight {arg1, arg2, ..} |
            Instruction::Mod {arg1, arg2, ..} | Instruction::BitwiseAnd {arg1, arg2, ..} |
            Instruction::BitwiseXor {arg1, arg2, ..} | Instruction::BitwiseOr {arg1, arg2, ..} |
            Instruction::LogicalAnd {arg1, arg2, ..} | Instruction::LogicalOr {arg1, arg2, ..} |
            Instruction::Equal {arg1, arg2, ..} | Instruction::NotEqual {arg1, arg2, ..} | 
            Instruction::Greater {arg1, arg2, ..} | Instruction::GreaterEqual {arg1, arg2, ..} |
            Instruction::Less {arg1, arg2, ..} | Instruction::LessEqual {arg1, arg2, ..} => {
                return vec![arg1.clone(), arg2.clone()];
            },
            Instruction::Assign { arg1, .. } | Instruction::Complement { arg1, .. } |
            Instruction::Neg { arg1, .. } | Instruction::Not { arg1, .. } => {
                return vec![arg1.clone()];  
            },
            Instruction::IfZero { cond, .. } => {
                return vec![cond.clone()]
            }          
            _ => Vec::new(),
        }
    }

}