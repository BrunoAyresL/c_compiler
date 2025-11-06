use crate::irgen::Operand;


pub enum Instruction {

    Label(String),

    Goto(String),


    IfZero {cond: Operand, label: String},
    Assign {dest: Operand, arg1: Operand},

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

            Instruction::IfZero { cond, label } => {
                format!("   IfZero {} Goto {}", cond.print(), label)
            },

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
}