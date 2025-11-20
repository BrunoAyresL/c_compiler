use std::collections::HashMap;

use crate::{intermediate::{frame::Frame, instruction::Instruction, irgen::Operand}, optimizer::liveness::Variable, parser::node::ConstValue};


// para cada instrução:
// pre-busca: se for var spilled, fazer LOAD (stack -> reg) antes
// seleção: TAC -> Asm
// armazenamento: se o resultado for alocado na stack, fazer STORE (reg -> stack)
// tac: t0 = a + b, a = %r10, b = -24(%rbp)
// movq -24(%rbp), %r11
// movq %r10, %rax
// addq %r11, %rax
// movq %rax, (endereço de t0) 

const WINDOWS_REGISTERS: [&str; 14] = [
    "%r8", "%r9", "%r10", "%r11", "%rcx", "%rdx", "%r14", "%r15",
    "%rbx", "%rsi", "%rdi", "%rax", "%r12", "%r13", 
];


pub struct AsmGenerator {
    curr: usize,
    instructions: Vec<Instruction>,
    frame: Frame,
    variables: HashMap<String, Variable>,
    spill: HashMap<String, i32>,
    reg_names: Vec<String>,    
    pub assembly: Vec<AsmInstruction>,
}


pub fn new_asm_generator(instructions: Vec<Instruction>, frame: Frame, 
    variables: HashMap<String, Variable>, spill: HashMap<String, i32>) -> AsmGenerator {
        let reg_names = WINDOWS_REGISTERS
        .iter().map(|&s| s.to_string()).collect();
        AsmGenerator { 
            curr: 0, 
            instructions, frame, 
            variables, spill, 
            reg_names,
            assembly: Vec::new()
    }
}
#[derive(Clone)]
pub enum Location {
    Reg(String),
    Stack(i32),
    Imm(String),
}
impl Location {
    pub fn to_string(&self) -> String {
        match self {
            Location::Reg(s) => s.clone(),
            Location::Imm(s) => {
                format!("${s}")
            },
            Location::Stack(offset) => {
                format!("{offset}(%rbp)")
            }
        }
    }
}

pub enum AsmInstruction {
    Ret,
    Label(String),

    Jmp(String),
    Je(String),
    Jne(String),
    Jl(String),
    Jle(String),
    Ja(String),
    Jae(String),

    SetE(Location),
    SetNE(Location),
    SetL(Location),
    SetLE(Location),
    SetG(Location),
    SetGE(Location),

    MovZbl(Location, Location),
    Mov(Location, Location),

    Cmp(Location, Location),
    Add(Location, Location),
    Sub(Location, Location),
    Push(Location),
    Pop(Location),
    
}


impl AsmGenerator {

    pub fn print_asm(&self) -> String {
        let mut s = format!(".globl {}\n", self.frame.name);
        for inst in &self.assembly {
            s.push_str(format!("{}\n", inst.to_string()).as_str());
        }
        s
    }

    fn emit(&mut self, i: AsmInstruction) {
        self.assembly.push(i);
    }

    fn curr_instruction(&self) -> &Instruction {
        &self.instructions[self.curr]
    }

    fn peek(&self) -> &Instruction {
        if self.curr + 1 < self.instructions.len() {
            return &self.instructions[self.curr + 1];
        }
        return self.curr_instruction();
    }

    fn next_instruction(&mut self) -> bool {
        if let Instruction::EndFunc = self.curr_instruction() {
            return false;
        }
        self.curr += 1;
        if self.curr >= self.instructions.len() {
            return false;
        }
        true
    }
    
    pub fn generate_assembly(&mut self) {
        let instructions = self.instructions.clone();
        let end_label = format!("{}_end", self.frame.name.clone());
        let rax = Location::Reg("%rax".to_string());

        loop {
            match self.curr_instruction().clone() {
                Instruction::Label(l) => {
                  self.emit(AsmInstruction::Label(l.clone()));  
                },
                Instruction::Goto(l) => {
                    self.emit(AsmInstruction::Jmp(l.clone()));
                },
                Instruction::Return { dest } => {
                    let dest = self.operand_to_reg(dest);
                    self.emit(AsmInstruction::Mov(dest, rax.clone()));
                    self.emit(AsmInstruction::Jmp(end_label.clone()));
                },
                Instruction::BeginFunc(_) => {
                    self.emit(AsmInstruction::Push(Location::Reg("%rbp".to_string())));
                    
                    self.emit(AsmInstruction::Mov(Location::Reg("%rsp".to_string()), 
                    Location::Reg("%rbp".to_string())));

                    let stack_space = self.spill.len() * 8;
                    if stack_space > 0 {
                        self.emit(AsmInstruction::Sub(Location::Imm(format!("{stack_space}")),
                        Location::Reg("%rsp".to_string())));
                    }
                },
                Instruction::EndFunc => {
                    self.emit(AsmInstruction::Label(end_label.clone()));
                    self.emit(AsmInstruction::Mov(Location::Reg("%rbp".to_string()), 
                    Location::Reg("%rsp".to_string())));
                    self.emit(AsmInstruction::Pop(Location::Reg("%rbp".to_string())));
                    self.emit(AsmInstruction::Ret);

                },
                Instruction::Assign { dest, arg1 } => {
                    let src = self.operand_to_reg(arg1);
                    let dest = self.operand_to_reg(dest);
                    if let Location::Stack(_) = src && let Location::Stack(_) = dest {
                        self.emit(AsmInstruction::Mov(src, Location::Reg("%rax".to_string())));
                        self.emit(AsmInstruction::Mov(Location::Reg("%rax".to_string()), dest));
                    } else {
                        self.emit(AsmInstruction::Mov(src, dest));
                    }
                    

                },
                Instruction::Equal { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Cmp(b, rax.clone())); 

                    if let Instruction::IfZero{ label, ..} = self.peek() {
                        self.emit(AsmInstruction::Jne(label.clone()));
                        self.next_instruction();
                    } else {
                        let al = Location::Reg("%al".to_string());
                        self.emit(AsmInstruction::SetE(al.clone()));
                        self.emit(AsmInstruction::MovZbl(al, Location::Reg("%eax".to_string())));
                        self.emit(AsmInstruction::Mov(rax.clone(), dest));
                    }
                },
                Instruction::NotEqual { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Cmp(b, rax.clone())); 

                    if let Instruction::IfZero{ label, ..} = self.peek() {
                        self.emit(AsmInstruction::Je(label.clone()));
                        self.next_instruction();
                    } else {
                        let al = Location::Reg("%al".to_string());
                        self.emit(AsmInstruction::SetNE(al.clone()));
                        self.emit(AsmInstruction::MovZbl(al, Location::Reg("%eax".to_string())));
                        self.emit(AsmInstruction::Mov(rax.clone(), dest));
                    }
                }
                Instruction::Greater { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Cmp(b, rax.clone())); 

                    if let Instruction::IfZero{ label, ..} = self.peek() {
                        self.emit(AsmInstruction::Jle(label.clone()));
                        self.next_instruction();
                    } else {
                        let al = Location::Reg("%al".to_string());
                        self.emit(AsmInstruction::SetG(al.clone()));
                        self.emit(AsmInstruction::MovZbl(al, Location::Reg("%eax".to_string())));
                        self.emit(AsmInstruction::Mov(rax.clone(), dest));
                    }

                }
                Instruction::GreaterEqual { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Cmp(b, rax.clone())); 

                    if let Instruction::IfZero{ label, ..} = self.peek() {
                        self.emit(AsmInstruction::Jl(label.clone()));
                        self.next_instruction();
                    } else {
                        let al = Location::Reg("%al".to_string());
                        self.emit(AsmInstruction::SetGE(al.clone()));
                        self.emit(AsmInstruction::MovZbl(al, Location::Reg("%eax".to_string())));
                        self.emit(AsmInstruction::Mov(rax.clone(), dest));
                    }
                }
                Instruction::Less { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Cmp(b, rax.clone())); 

                    if let Instruction::IfZero{ label, ..} = self.peek() {
                        self.emit(AsmInstruction::Jae(label.clone()));
                        self.next_instruction();
                    } else {
                        let al = Location::Reg("%al".to_string());
                        self.emit(AsmInstruction::SetL(al.clone()));
                        self.emit(AsmInstruction::MovZbl(al, Location::Reg("%eax".to_string())));
                        self.emit(AsmInstruction::Mov(rax.clone(), dest));
                    }
                }
                Instruction::LessEqual { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Cmp(b, rax.clone())); 

                    if let Instruction::IfZero{ label, ..} = self.peek() {
                        self.emit(AsmInstruction::Ja(label.clone()));
                        self.next_instruction();
                    } else {
                        let al = Location::Reg("%al".to_string());
                        self.emit(AsmInstruction::SetLE(al.clone()));
                        self.emit(AsmInstruction::MovZbl(al, Location::Reg("%eax".to_string())));
                        self.emit(AsmInstruction::Mov(rax.clone(), dest));
                    }
                }
                

                Instruction::Add { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);


                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Add(b, Location::Reg("%rax".to_string())));

                    if let Location::Reg(s) = &dest && s.contains("%rax") {
                        continue;
                    }

                    self.emit(AsmInstruction::Mov(rax.clone(), dest));

                },
                Instruction::Sub { dest, arg1, arg2 } => {
                    let a = self.operand_to_reg(arg1);
                    let b = self.operand_to_reg(arg2);
                    let dest = self.operand_to_reg(dest);

                    self.emit(AsmInstruction::Mov(a, rax.clone()));
                    self.emit(AsmInstruction::Sub(b, rax.clone()));

                    if let Location::Reg(s) = &dest && s.contains("%rax") {
                        self.next_instruction();
                        continue;
                    }

                    self.emit(AsmInstruction::Mov(rax.clone(), dest));

                },


                _ => (),
            }
            if !self.next_instruction() { break; }
        }
        
    }


    fn operand_to_reg(&self, op: Operand) -> Location {
        if let Operand::Const(c) = op {
            return Location::Imm(c.to_string()); // mudar depois
        }
        let var = &self.variables[&op.print()];
        if var.spilled {
            return Location::Stack(self.spill[&var.name]);
        } 
        return Location::Reg(self.reg_names[var.register_id].clone());
    }


}


impl AsmInstruction {
    pub fn to_string(&self) -> String {
        match self {
            AsmInstruction::Label(l) => {
                format!("{l}:")
            },
            AsmInstruction::Jmp(l) => {
                format!("\tjmp {l}")
            },
            AsmInstruction::Je(l) => {
                format!("\tje {l}")
            },
            AsmInstruction::Jne(l) => {
                format!("\tjne {l}")
            },
            AsmInstruction::Jl(l) => {
                format!("\tjl {l}")
            },
            AsmInstruction::Jle(l) => {
                format!("\tjle {l}")
            },
            AsmInstruction::Ja(l) => {
                format!("\tja {l}")
            },
            AsmInstruction::Jae(l) => {
                format!("\tjae {l}")
            },
            AsmInstruction::SetE(a) => {
                format!("\tsete {}", a.to_string())
            }, 
            AsmInstruction::SetNE(a) => {
                format!("\tsetne {}", a.to_string())
            },
            AsmInstruction::SetL(a) => {
                format!("\tsetl {}", a.to_string())
            },
            AsmInstruction::SetLE(a) => {
                format!("\tsetle {}", a.to_string())
            },
            AsmInstruction::SetG(a) => {
                format!("\tsetg {}", a.to_string())
            },
            AsmInstruction::SetGE(a) => {
                format!("\tsetge {}", a.to_string())
            },          
            AsmInstruction::Ret => {
                format!("\tret")
            },
            AsmInstruction::Cmp(a, b) => {
                format!("\tcmpq {}, {}", a.to_string(), b.to_string())
            },
            AsmInstruction::Mov(a, b) => {
                format!("\tmovq {}, {}", a.to_string(), b.to_string())
            },
            AsmInstruction::MovZbl(a, b) => {
                format!("\tmovzbl {}, {}", a.to_string(), b.to_string())
            },
            AsmInstruction::Add(a, b) => {
                format!("\taddq {}, {}", a.to_string(), b.to_string())
            },
            AsmInstruction::Sub(a, b) => {
                format!("\tsubq {}, {}", a.to_string(), b.to_string())
            },
            AsmInstruction::Push(a,) => {
                format!("\tpushq {}", a.to_string())
            },
            AsmInstruction::Pop(a) => {
                format!("\tpopq {}", a.to_string())
            },
        }
    }
}