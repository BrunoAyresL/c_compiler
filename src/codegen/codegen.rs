use std::collections::HashMap;

use crate::{intermediate::{frame::Frame, instruction::Instruction, irgen::Operand}, optimizer::liveness::Variable, parser::node::ConstValue};


// para cada instrução:
// pre-busca: se for var spilled, fazer LOAD (stack -> reg) antes
// seleção: TAC -> Asm
// armazenamento: se o resultado for alocado na stack, fazer STORE (reg -> stack)





pub struct AsmGenerator {
    instructions: Vec<Instruction>,
    frame: Frame,
    variables: HashMap<String, Variable>,
    spill: HashMap<String, i32>,
    reg_names: Vec<String>,    
    assembly: Vec<AsmInstruction>,
}

pub fn new_asm_generator(instructions: Vec<Instruction>, frame: Frame, 
    variables: HashMap<String, Variable>, spill: HashMap<String, i32>) -> AsmGenerator {
        let mut reg_names = ["rbx", "r12", "r13", "r14", "r15"];
        AsmGenerator { instructions, frame, variables, spill, 
            reg_names: vec![reg_names.iter_mut().map(|s| s.to_string()).collect()],
            assembly: Vec::new()
    }
}

enum AsmOperand {
    Reg(String),
    Spill(i32),
    Imm(String),
}

enum AsmInstruction {
    Ret,
    Label(String),
    Mov(AsmOperand, AsmOperand),
    Add(AsmOperand, AsmOperand),
    Sub(AsmOperand, AsmOperand),
    Push(AsmOperand),
    Pop(AsmOperand),
    
}


impl AsmGenerator {

    fn emit(&mut self, instruction: AsmInstruction) {
        self.assembly.push(instruction);
    }

    pub fn generate_assembly(&mut self) {
        let instructions = self.instructions.clone();
        for inst in instructions {
            match inst {
                Instruction::Label(l) => {
                  self.emit(AsmInstruction::Label(l.clone()));  
                },
                Instruction::Return { dest } => {
                    self.emit(AsmInstruction::Mov(self.operand_to_reg(dest), 
                    AsmOperand::Reg("%eax".to_string())));
                    self.emit(AsmInstruction::Ret);
                }
                Instruction::BeginFunc(_) => {
                    self.emit(AsmInstruction::Push(AsmOperand::Reg("%rbp".to_string())));
                    
                    self.emit(AsmInstruction::Mov(AsmOperand::Reg("%rsp".to_string()), 
                    AsmOperand::Reg("%rbp".to_string())));

                    let stack_space = self.spill.len() * 8;
                    if stack_space > 0 {
                        self.emit(AsmInstruction::Sub(AsmOperand::Imm(format!("{stack_space}")),
                        AsmOperand::Reg("%rsp".to_string())));
                    }
                },
                Instruction::Assign { dest, arg1 } => {
                    self.emit(AsmInstruction::Mov(self.operand_to_reg(arg1), self.operand_to_reg(dest)));
                },
                Instruction::Add { dest, arg1, arg2 } => {

                }

                _ => (),
            }
        }
    }


    fn operand_to_reg(&self, op: Operand) -> AsmOperand {
        if let Operand::Const(c) = op {
            return AsmOperand::Imm(c.to_string()); // mudar depois
        }
        let var = &self.variables[&op.print()];
        if var.spilled {
            return AsmOperand::Spill(self.spill[&var.name]);
        } 
        return AsmOperand::Reg(self.reg_names[var.register_id].clone());
    }


}