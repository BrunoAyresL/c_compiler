// Live Range 
// Interference
// Node, Edge

use std::{collections::HashMap};

use crate::{intermediate::{frame::Frame, instruction::Instruction}, optimizer::{cfg::Block, liveness::Variable}};


static REG_COUNT: usize = 8;

pub struct Allocator {
    register_count: usize,
    variables: HashMap<String, Variable>,
}

pub fn new_allocator(variables: HashMap<String, Variable>) -> Allocator {
    Allocator { 
        register_count: REG_COUNT,
        variables
    }
}


impl Allocator {

    pub fn coloring(&mut self) {
        //let mut spill = Vec::new();
        let mut stack = Vec::new();

        while !self.variables.is_empty() {
            if let Some((_, var)) = self.variables.iter()
            .find(|var|
            var.1.edges.len() < self.register_count) {
                stack.push((var.clone(), false));
                self.variables.remove(&var.name.clone());
            }
            
            

        }


    }







/* 
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
    */
}