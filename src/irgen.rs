

// array
// load arr[index]
// store arr[index]

// array
// type
// size


use crate::{instruction::Instruction, node::{ConstValue, ParserNode}};

pub struct CodeGen {
    instructions: Vec<Instruction>,
    temp_count: usize,
    label_count: usize,
}

pub fn new_codegen() -> CodeGen {
    CodeGen {
        instructions: Vec::new(),
        temp_count: 0,
        label_count: 0,
    }
}

#[derive(Clone)]
pub enum Operand {
    Const(ConstValue),
    Var(String),
    Temp(String),
    None,
}
impl Operand {
    pub fn print(&self) -> String {
        match self {
            Operand::Const(val) => {
                format!("{}", val.to_string())
            },
            Operand::Var(v) => {
                v.clone()
            },
            Operand::Temp(t) => {
                t.clone()
            },
            Operand::None => String::new()
        }
    }
}


impl CodeGen {
    fn emit(&mut self, tac: Instruction) {
        self.instructions.push(tac);
    }

    pub fn cgen(&mut self, node: &ParserNode) -> Operand {
        match node {
            ParserNode::Block(nodes) => {
                for n in nodes {
                    self.cgen(n);
                }
                Operand::None
            },

            ParserNode::Declare { ident, exp, ntype:_ } => {
                match exp {
                    Some(n) => {
                        let dest = self.cgen(ident);     
                        let arg1 = self.cgen(n);
                        self.emit(Instruction::Assign { dest: dest.clone(), arg1 });
                        dest 
                    },
                    None => Operand::None,
                }
            }

            ParserNode::FuncDecl { ident, args:_, block, size, ntype:_ } => {
                self.emit(Instruction::Label(ident.to_string()));
                let begin_index = self.instructions.len();
                self.emit(Instruction::BeginFunc(*size));

                let prev_count = self.temp_count;
                self.cgen(block);
                self.emit(Instruction::EndFunc);
                let curr_count = self.temp_count; 
                if let Instruction::BeginFunc(ref mut s) = self.instructions[begin_index] {
                    *s += (curr_count - prev_count) * 4;
                } else {
                    panic!("instruction is not BeginFunc")
                }

                Operand::None
            },

            ParserNode::FuncCall { ident, args } => {

                for arg in args {
                    let mut t1 = self.cgen(arg);
                    match t1 {
                        Operand::Const(num) => {
                            t1 = self.new_temp();
                            self.emit(Instruction::Assign { dest: t1.clone(), arg1: Operand::Const(num) });
                        },
                        _ => (),
                    }
                    self.emit(Instruction::PushParam(t1));
                }
                self.emit(Instruction::LCall(ident.clone()));
                if args.len() > 0 {
                    let size = args.len() * 4;
                    self.emit(Instruction::PopParams(size));
                }
                Operand::None
            },

            // statements
            ParserNode::Assign { left, right } => {
                let dest = self.cgen(left);     
                let arg1 = self.cgen(right);
                self.emit(Instruction::Assign { dest: dest.clone(), arg1 });
                dest      
            },

            ParserNode::If { cond, block, else_stmt } => {
                match else_stmt {
                    Some(n) => {
                        let cond = self.cgen(cond);
                        let end_label = self.new_label();
                        let else_label = self.new_label();
                        self.emit(Instruction::IfZero { cond, label: else_label.clone() });
                        self.cgen(&block);
                        self.emit(Instruction::Goto(end_label.clone()));
                        self.emit(Instruction::Label(else_label));
                        self.cgen(&n);
                        self.emit(Instruction::Label(end_label));
                    },
                    None => {
                        let cond = self.cgen(cond);
                        let end_label = self.new_label();
                        self.emit(Instruction::IfZero { cond, label: end_label.clone() });
                        self.cgen(&block);
                        self.emit(Instruction::Label(end_label));
                    }
                } 
                Operand::None
            },
            ParserNode::Return { exp } => {
                let mut dest = self.cgen(exp);
                match dest {
                    Operand::Const(num) => {
                        dest = self.new_temp();
                        self.emit(Instruction::Assign { dest: dest.clone(), arg1: Operand::Const(num) });
                    },
                    _ => (),
                }
                self.emit(Instruction::Return { dest });
                Operand::None
            }

            // logical
            ParserNode::LogicalOr { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::LogicalOr { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::LogicalAnd { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::LogicalAnd { dest: dest.clone(), arg1, arg2 });
                dest
            },

            // bitwise
            ParserNode::BitwiseOr { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::BitwiseOr { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::BitwiseXor { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::BitwiseXor { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::BitwiseAnd { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::BitwiseAnd { dest: dest.clone(), arg1, arg2 });
                dest
            },

            // equality
            ParserNode::Equal { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Equal { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::NotEqual { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::NotEqual { dest: dest.clone(), arg1, arg2 });
                dest
            },

            // relational
            ParserNode::Greater { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Greater { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::GreaterEqual { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::GreaterEqual { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::Less { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Less { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::LessEqual { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::LessEqual { dest: dest.clone(), arg1, arg2 });
                dest
            },

            // shift
            ParserNode::ShiftLeft { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::ShiftLeft { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::ShiftRight { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::ShiftRight { dest: dest.clone(), arg1, arg2 });
                dest
            },
            
            // additive
            ParserNode::Add { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Add { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::Sub { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Sub { dest: dest.clone(), arg1, arg2 });
                dest
            },

            // term
            ParserNode::Mul { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Mul { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::Div { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Div { dest: dest.clone(), arg1, arg2 });
                dest
            },
            ParserNode::Mod { left, right } => {
                let arg1 = self.cgen(left);     
                let arg2 = self.cgen(right); 
                let dest = self.new_temp();  
                self.emit(Instruction::Mod { dest: dest.clone(), arg1, arg2 });
                dest
            },

            // unary
            ParserNode::Neg { val} => {
                let dest = self.new_temp();
                let arg1 = self.cgen(val);    
                self.emit(Instruction::Neg { dest: dest.clone(), arg1 });
                dest
            },
            ParserNode::Complement { val} => {
                let dest = self.new_temp();
                let arg1 = self.cgen(val);    
                self.emit(Instruction::Complement { dest: dest.clone(), arg1 });
                dest
            },
            ParserNode::Not { val} => {
                let dest = self.new_temp();
                let arg1 = self.cgen(val);    
                self.emit(Instruction::Not { dest: dest.clone(), arg1 });
                dest
            },

            // factor
            ParserNode::Var{ ident, ntype: _} => Operand::Var(ident.clone()),
            ParserNode::Const(val) => Operand::Const(val.clone()),
            ParserNode::SubExp { val} => {
                self.cgen(val)
            },


            // other
            ParserNode::Expression( nodes) => {
                for n in nodes.iter() {
                    self.cgen(n);
                }
                Operand::None
            },
            _ => {
                println!("NOT IMPLEMENTED: {:?}", node);
                Operand::None
            },
        }
    }

    fn new_temp(&mut self) -> Operand {
        let t = format!("t{}", self.temp_count);
        self.temp_count += 1;
        Operand::Temp(t)
    }
    fn new_label(&mut self) -> String {
        let l = format!("L{}", self.label_count);
        self.label_count += 1;
        l
    }

    pub fn print_instructions(&self) -> String {
        let mut s = String::new();
        for inst in self.instructions.iter() {
            s.push_str(&inst.print().as_str());
            s.push_str("\n");
        }
        s
    }

}

