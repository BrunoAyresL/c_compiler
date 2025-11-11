// Live Range 
// Interference
// Node, Edge

use std::collections::HashMap;

use crate::{frame::Frame, instruction::{Instruction}, irgen::Operand};


pub struct Allocator {
    instructions: Vec<Instruction>,
    frames: HashMap<String, Frame>,
    live_ranges: Vec<LiveRange>,
}

pub fn new_allocator(instructions: Vec<Instruction>, frames: HashMap<String, Frame>) -> Allocator {
    Allocator { 
        instructions,
        frames,
        live_ranges: Vec::new(),
    }
}
#[derive(Debug)]
pub struct LiveRange {
    name: String,
    live_in: usize,
    live_out: usize,
}

pub struct LiveNow {
    live_ranges: Vec<LiveRange>,
    
}

impl Allocator {

    pub fn get_liveness(&mut self) -> &Vec<LiveRange> {
        let mut begin_pos = 0;
        let mut blocks = Vec::new();
        for (i, inst) in self.instructions.iter_mut().enumerate() {
            if let Instruction::BeginFunc(_) = inst {
               begin_pos = i;
            }
            if let Instruction::EndFunc = inst {
                blocks.push((begin_pos, i));
            }
        }

        for (begin, end) in blocks {
            self.analyze_block(begin, end);
        }
        &self.live_ranges
    }

    fn analyze_block(&mut self, begin_pos: usize, end_pos: usize) {
        let mut live_out: HashMap<String, usize> = HashMap::new();

        let mut curr_pos = end_pos;

        while curr_pos > begin_pos {
            let inst = &self.instructions[curr_pos];
            let def = inst.def();
            if let Some(Operand::Temp(s)) = def {

                if let Some(live_out) = live_out.remove(&s) {
                    self.live_ranges.push(LiveRange {
                        name: s, 
                        live_in: curr_pos, 
                        live_out,
                    });
                }

            }
            let uses = inst.uses();
            for op in uses {
                if let Operand::Temp(s) = op {
                    if live_out.contains_key(&s) {
                        continue;
                    } else {
                        live_out.insert(s, curr_pos);
                    }
                }
            }
            
            curr_pos -= 1;
        }
        
    }

    pub fn allocate_registers() {

    }
}