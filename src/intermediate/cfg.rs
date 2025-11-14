use core::fmt;
use std::collections::HashMap;

use crate::intermediate::{cfg, frame::Frame, instruction::Instruction};

#[derive(Debug)]
pub struct ControlFlowGraph {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    id: usize,
    label: Option<String>,
    first: usize, // BeginFunc, Label, after Ifzero, Return, Call, etc
    last: usize, // Ifzero, Goto, Return, EndFunc
    edges: Vec<usize>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})\t{}-{}\t ->\t{:?}\t{}", self.id, 
        self.first, 
        self.last, 
        self.edges,
        self.label.as_deref().unwrap_or("")
        )
    }
}

struct CFGBuilder {
    function_name: String,
    instructions: Vec<Instruction>,
    start: usize,
    curr: usize,
    label: Option<String>,
    count: usize,
    blocks: Vec<Block>,
}

fn new_cfg_builder(function_name: String, instructions: Vec<Instruction>) -> CFGBuilder {
    let mut curr = 0;
    for inst in &instructions {
        if let Instruction::BeginFunc(_) = inst {
            break;
        }
        curr += 1;
    }
    CFGBuilder { function_name, instructions, start: curr, curr, label: None, count: 0, blocks: Vec::new() }
}

impl CFGBuilder {
    fn curr_instruction(&mut self) -> &Instruction {
        return self.instructions.get(self.curr).unwrap();
    }
    fn read_instruction(&mut self) -> &Instruction {
        self.curr += 1;
        
        if self.curr >= self.instructions.len() {
            return &Instruction::EndFunc;
        } else {
            return self.instructions.get(self.curr).unwrap();
        }
    }
    fn build_block(&mut self) {
        let block = Block {
            id: self.count,
            label: self.label.clone(),
            first: self.start,
            last: self.curr,
            edges: Vec::new(),
        };
        self.count += 1;
        
        if self.start < self.instructions.len() {
            self.start = self.curr + 1;
        }
        self.label = None;
        self.blocks.push(block);
    }
    fn check_instruction(&mut self) -> bool {
        match self.curr_instruction() {
            Instruction::BeginFunc(_) => {
                self.start = self.curr + 1;
                
            },
            Instruction::Label(l) => {
                self.label = Some(l.clone());
                self.start = self.curr + 1;
            },
            Instruction::IfZero { .. } | Instruction::Goto(..) => {
                self.build_block();
            },
            Instruction::EndFunc => {
                self.build_block();
                return false;
            },
            _ => (),
        }
        self.read_instruction();
        true
    }

    fn build_function_blocks(&mut self) -> &Vec<Block> {
        while self.check_instruction() {
        }
        return &self.blocks;
    }

    fn get_next_block_id(&self, label: &String) -> usize {
        (self.blocks
        .iter()
        .find(|b| b.label.clone()
        .is_some_and(|l| l == *label)))
        .expect(format!("ERROR - label '{}' not found", label).as_str())
        .id
    }
    
    fn link_block_edges(&mut self) {
        for i in   0..self.blocks.len() {
            let mut edges = Vec::new();
            let block_last = self.blocks[i].last;
            let last_instruction = &self.instructions[block_last];

            match last_instruction {
                Instruction::IfZero { label, .. } => {
                    let next_id = self.get_next_block_id(label);
                    edges.push(next_id);
                    if self.blocks[i].id < self.count + 1{
                        let next_id = self.blocks[i].id + 1;
                        edges.push(next_id);
                    }
                }, 
                Instruction::Goto(label) => {
                    let next_id = self.get_next_block_id(label);
                    edges.push(next_id);
                },
                Instruction::EndFunc => {

                }
                _ => panic!("link_block_edges: unexpected {:?}", last_instruction),
            }
            self.blocks[i].edges = edges;
        }
    }
    
    pub fn build(&mut self) -> ControlFlowGraph {
        ControlFlowGraph { blocks: self.blocks.clone() }
    }

}


pub fn create_cfgs(frames: &HashMap<String, Frame>, instructions: &Vec<Instruction>) -> Vec<ControlFlowGraph> { // tbd
    let mut cfgs: Vec<ControlFlowGraph> = Vec::new();
    for (frame_name,_) in frames {
        let mut start = 0;
        let mut end = 0;
        for (i, inst) in instructions.iter().enumerate() {
            if let Instruction::Label(l) = inst && *l == *frame_name {
                start = i + 1;
            } else if let Instruction::EndFunc = inst {
                end = i;
            }
        }
        if start == end { continue; }
        
        let mut cfg_builder = new_cfg_builder(frame_name.clone(), 
            instructions[start..=end].to_vec());
        cfg_builder.build_function_blocks();
        cfg_builder.link_block_edges();
        cfgs.push(cfg_builder.build());
    }
    return cfgs;
}

 
