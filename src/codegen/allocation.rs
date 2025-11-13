// Live Range 
// Interference
// Node, Edge

use std::{collections::HashMap, vec};

use crate::{intermediate::frame::Frame, intermediate::instruction::{Instruction}, intermediate::irgen::Operand};


pub struct Allocator {
    instructions: Vec<Instruction>,
    frames: HashMap<String, Frame>,
    pub live_ranges: Vec<LiveRange>, // MUDAR
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
    pub register_id: usize,
}

impl LiveRange {
    pub fn check_interference(&self, other: &LiveRange) -> bool {
        self.live_in <= other.live_in && self.live_out > other.live_in ||
        self.live_in >= other.live_in && self.live_in < other.live_out
    }
}

pub struct LiveNow {
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
                        register_id: 0,
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

    pub fn allocate_registers(&mut self) {
        // graph
        let mut lv_table : Vec<Vec<usize>> = vec![vec![]; self.live_ranges.len()];

        for (i, lr1) in self.live_ranges.iter().enumerate() {
            for (ii, lr2) in self.live_ranges.iter().enumerate() {
                if i == ii { continue; }
                if lr1.check_interference(lr2) {
                    lv_table[i].push(ii);
                }
            }
        }
        // coloring

        for (i, edges) in lv_table.iter().enumerate() {
            let mut reg = 0;
            let mut used_regs = Vec::new();
            for ii in edges {
                let nb = &self.live_ranges[*ii];
                used_regs.push(nb.register_id);
                if nb.register_id == reg {
                    while used_regs.contains(&reg) {
                        reg += 1;
                    }
                }
            }
            self.live_ranges[i].register_id = reg;
        }

    }
}