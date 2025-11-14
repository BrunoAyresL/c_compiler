

// array
// load arr[index]
// store arr[index]

// array
// type
// size


use std::collections::HashMap;


use crate::{intermediate::frame::Frame, intermediate::instruction::Instruction, parser::node::{ConstValue, ParserNode}};

pub struct CodeGen {
    pub instructions: Vec<Instruction>,
    pub frames: HashMap<String, Frame>,
    temp_count: usize,
    label_count: usize,
}

pub fn new_codegen(frames: HashMap<String, Frame>) -> CodeGen {
    CodeGen {
        instructions: Vec::new(),
        frames,
        temp_count: 0,
        label_count: 0,
    }
}

#[derive(Debug, Clone, PartialEq)]
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

            ParserNode::FuncDecl { ident, args:_, block, ntype:_ } => {
                self.emit(Instruction::Label(ident.to_string()));
                let frame = self.frames.get(ident.to_string().as_str()).unwrap();
                self.emit(Instruction::BeginFunc(frame.locals_size));

                self.cgen(block);
                self.emit(Instruction::EndFunc);
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
                        let else_label = self.new_label();
                        let mut end_label = self.new_label();
                        self.emit(Instruction::IfZero { cond, label: else_label.clone() });
                        self.cgen(&block);
                        let goto_index = self.instructions.len();
                        self.emit(Instruction::Goto(end_label.clone()));
                        self.emit(Instruction::Label(else_label));
                        self.cgen(&n);
                        if let Instruction::Label(l) = self.instructions.last().unwrap() {
                            let prev_label = l.clone();
                            if let Instruction::Goto(goto_label) = &mut self.instructions[goto_index] {
                                *goto_label = prev_label.clone();
                                self.label_count -= 1;
                                end_label = prev_label;
                                self.emit(Instruction::Goto(end_label.clone()));
                            }
                        } else {
                            self.emit(Instruction::Goto(end_label.clone()));
                            self.emit(Instruction::Label(end_label));
                        }
    
                    },
                    None => {
                        let cond = self.cgen(cond);
                        let end_label = self.new_label();
                        let if_index = self.instructions.len();
                        self.emit(Instruction::IfZero { cond, label: end_label.clone() });
                        self.cgen(&block);
                        if let Instruction::Label(l) = self.instructions.last().unwrap() {
                            let prev_label = l.clone();
                            if let Instruction::IfZero { label , ..} = &mut self.instructions[if_index] {
                                *label = prev_label;
                                self.label_count -= 1;
                            }
                        } else {
                            self.emit(Instruction::Label(end_label));
                        }
                        
                    }
                } 
                Operand::None
            },
            ParserNode::For { exp1, exp2, exp3, block } => {
                self.cgen(exp1);
                let exp2 = self.cgen(exp2);
                let end_label = self.new_label();
                self.emit(Instruction::IfZero { cond: exp2.clone(), label: end_label.clone() });
                let loop_label = self.new_label();
                self.emit(Instruction::Label(loop_label.clone()));
                self.cgen(block);
                self.cgen(exp3);
                self.emit(Instruction::IfZero { cond: exp2, label: end_label.clone() });
                self.emit(Instruction::Goto(loop_label));
                self.emit(Instruction::Label(end_label));
                Operand::None
            },
            ParserNode::While { cond, block } => {
                let cond = self.cgen(cond);
                let end_label = self.new_label();
                self.emit(Instruction::IfZero { cond: cond.clone(), label: end_label.clone() });
                let loop_label = self.new_label();
                self.emit(Instruction::Label(loop_label.clone()));
                self.cgen(block);
                self.emit(Instruction::IfZero { cond: cond, label: end_label.clone() });
                self.emit(Instruction::Goto(loop_label));
                self.emit(Instruction::Label(end_label));
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
            /* _ => {
                println!("NOT IMPLEMENTED: {:?}", node);
                Operand::None
            }, */
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

