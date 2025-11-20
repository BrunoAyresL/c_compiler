use core::fmt;
use std::collections::{HashMap, HashSet};

use crate::intermediate::{frame::Frame, instruction::Instruction};

enum TACError {
    UnexpectedInstruction { expected: String, found: String }
}

impl fmt::Display for TACError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TACError::UnexpectedInstruction{expected, found}
            => write!(f, "TACError: expected '{}' found  '{}'", expected, found),
        }
    }
}


#[derive(Debug)]
pub struct ControlFlowGraph {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: usize,
    label: Option<String>,
    first: usize, // BeginFunc, Label, after Ifzero, Return, Call, etc
    last: usize, // Ifzero, Goto, Return, EndFunc
    pub edges: Vec<usize>,

    pub live_in: HashSet<String>,
    pub live_out: HashSet<String>,
    pub def_set: HashSet<String>,
    pub use_set: HashSet<String>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id_string = format!("({})", self.id);
        let range_string = format!("{}-{}", self.first, self.last);
        let edges_string = format!("{:?}", self.edges);
        let label_string = self.label.as_deref().unwrap_or("-");
        write!(f, "{:^9} {:^9} ->  {:^12}  {:^9} |  IN {:<4?} OUT {:<4?} DEF {:<4?} USE {:<4?}", 
        id_string, 
        range_string,
        edges_string,
        label_string,
        self.live_in,
        self.live_out,
        self.def_set,
        self.use_set,
        )
    }
}
impl Block {
    pub fn get_range(&self) -> (usize, usize) {
        (self.first, self.last)
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
            live_in: HashSet::new(),
            live_out: HashSet::new(),
            def_set: HashSet::new(),
            use_set: HashSet::new()
        };

        self.count += 1;
        
        if self.start < self.instructions.len() {
            self.start = self.curr + 1;
        }
        self.label = None;
        self.blocks.push(block);
    }
    fn check_instruction(&mut self) -> bool {
        let start = self.start;
        let curr = self.curr;
        match self.curr_instruction() {
            Instruction::BeginFunc(_) => {
                self.start = self.curr;
                
            },
            Instruction::Label(l) => {
                if start < curr {
                    self.curr -= 1;
                    self.build_block();
                } else {
                    self.label = Some(l.clone());
                    self.start = self.curr;
                }
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

    fn get_next_block_id(&self, label: &String) -> Result<usize, TACError> {
        let found= self.blocks
        .iter()
        .find(|b| b.label.clone()
        .is_some_and(|l| l == *label));
        match found {
            Some(block) => Ok(block.id),
            None => Err(TACError::UnexpectedInstruction { 
                expected: label.clone(), 
                found: String::from("None") 
            } )
        }
    }
    
    fn link_block_edges(&mut self) -> Result<(), TACError>{
        for i in   0..self.blocks.len() {
            let mut edges = Vec::new();
            let block_last = self.blocks[i].last;
            let last_instruction = &self.instructions[block_last];

            match last_instruction {
                Instruction::IfZero { label, .. } => {
                    if self.blocks[i].id < self.count + 1 {
                        let next_id = self.blocks[i].id + 1;
                        edges.push(next_id);
                    }
                    let next_id = self.get_next_block_id(label)?;
                    edges.push(next_id);
                    
                }, 
                Instruction::Goto(label) => {
                    let next_id = self.get_next_block_id(label)?;
                    edges.push(next_id);
                },
                Instruction::EndFunc => {

                }
                _ => {
                    if self.blocks[i].id < self.count + 1 {
                        let next_id = self.blocks[i].id + 1;
                        edges.push(next_id);
                    }
                }
            }
            self.blocks[i].edges = edges;
        }
        Ok(())
    }
    
    pub fn build(&mut self) -> ControlFlowGraph {
        self.build_function_blocks();
        match self.link_block_edges() {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        
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
                start = i;
            } else if let Instruction::EndFunc = inst {
                end = i;
            }
        }
        if start == end { continue; }
        
        let mut cfg_builder = new_cfg_builder(frame_name.clone(), 
            instructions[start..=end].to_vec());
        cfgs.push(cfg_builder.build());
    }
    return cfgs;
}

 
