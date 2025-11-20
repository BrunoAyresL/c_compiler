use indexmap::{IndexMap, IndexSet};

use crate::{intermediate::{instruction::Instruction, irgen::Operand}, optimizer::cfg::Block};

pub struct LivenessAnalyzer {
    instructions: Vec<Instruction>,
    pub blocks: Vec<Block>,
    pub interference_graph: InterferenceGraph,
    pub inst_liveness: Vec<InstructionLiveness>,
    pub callstarts: Vec<Vec<Operand>>,
}

pub fn new_liveness_analyzer(instructions: Vec<Instruction>, blocks: Vec<Block>) -> LivenessAnalyzer {
    LivenessAnalyzer { 
        inst_liveness: vec![InstructionLiveness {
            live_in: IndexSet::new(),
            live_out: IndexSet::new()
        }; instructions.len()], 
        instructions, blocks, 
        interference_graph: InterferenceGraph { variables: IndexMap::new(), edges: IndexMap::new() },
        callstarts: Vec::new(),
    }
        
}
#[derive(Clone, Debug)]
pub struct InterferenceGraph {
    pub variables: IndexMap<String, Variable>,
    pub edges: IndexMap<String, IndexSet<String>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Variable {
    pub name: String,
    pub register_id: usize, 
    pub spilled: bool,
}

/* impl LiveRange {
    fn intersect(&self, other: &LiveRange) -> bool { // errado?
        self.start < other.end && other.start < self.end
    }

} */
#[derive(Debug, Clone)]
pub struct InstructionLiveness {
    live_in: IndexSet<String>,
    live_out: IndexSet<String>
}


impl LivenessAnalyzer {

    pub fn gen_live_out(&mut self) {
        
        self.generate_use_def();
        let mut changed = true;
        while changed {
            changed = false;
            for i in (0..self.blocks.len()).rev() {
                self.blocks[i].live_in = self.block_live_in(i);
                if self.block_live_out(i) {
                    changed = true;
                }
            }
        }
    }

    fn block_live_in(&self, id: usize) -> IndexSet<String> {
        let block = &self.blocks[id];
        
        let prop: IndexSet<String> = block.live_out
        .difference(&block.def_set)
        .cloned()
        .collect();

        let mut live_in = block.use_set.clone();

        live_in.extend(prop);

        live_in
    }

    fn block_live_out(&mut self, id: usize) -> bool {
        let block =  &self.blocks[id];
        let prev_live_out = block.live_out.clone();
        let mut live_out = IndexSet::new();
        for successor_id in &block.edges {
            live_out.extend(self.blocks[*successor_id].live_in.clone());
        }
        self.blocks[id].live_out = live_out;

        return prev_live_out != self.blocks[id].live_out;
    }

    fn generate_use_def(&mut self) {
        for i in 0..self.blocks.len() {
            let mut use_set = IndexSet::new();
            let mut def_set = IndexSet::new();

            let (a, b) = self.blocks[i].get_range();
            assert!(a <= b, "invalid range");
            assert!(b < self.instructions.len(), "b out of bounds");
            for ii in a..=b {
                
                if let Some(op) = self.instructions[ii].def() {
                    
                    def_set.insert(op.print());
                }
                let operands = self.instructions[ii].uses();
                use_set.extend(operands.iter().filter_map(|x| {
                    if def_set.contains(&x.print()) { return None }
                    if let Operand::Var(s) = x {
                        Some(s.clone())
                    } else if let Operand::Temp(s) = x {
                        Some(s.clone())   
                    }else {
                        None
                    }
                }));
                
            } 
            self.blocks[i].def_set = def_set;
            self.blocks[i].use_set = use_set;
        }
    }

    pub fn gen_inst_live_out(&mut self) {
        for i in 0..self.blocks.len() {
            let (a, b) = self.blocks[i].get_range();
            self.inst_liveness[b].live_out = self.blocks[i].live_out.clone();

            let mut changed = true;
            while changed {
                changed = false;
                for ii in (a..=b).rev() {
                    let inst = &self.instructions[ii];
                    let prev_live_in = self.inst_liveness[ii].live_in.clone();
                    let prev_live_out = self.inst_liveness[ii].live_out.clone();
                    
                    // live out
                    if ii != b && ii < self.inst_liveness.len() {
                        self.inst_liveness[ii].live_out = self.inst_liveness[ii+1].live_in.clone();
                    }

                    // live in
                    let uses = inst.uses();
                    self.inst_liveness[ii].live_in.clear();
                    self.inst_liveness[ii].live_in.extend(uses.iter().filter_map(|op| {
                        if let Operand::Const(_) = op {
                            return None
                        } 
                        return Some(op.print())
                    }));
                    let def = inst.def();
                    let mut prop: IndexSet<String> = self.inst_liveness[ii].live_out.clone();
                    if let Some(op) = def {
                        prop.swap_remove(&op.print());
                    }
                    self.inst_liveness[ii].live_in.extend(prop);
                    // live out

                    if prev_live_in != self.inst_liveness[ii].live_in ||
                        prev_live_out != self.inst_liveness[ii].live_out {
                            changed = true;
                        } 
                }
            }
        
        }
    }

 
    pub fn create_interference_graph(&mut self) {
        for i in 0..self.instructions.len() {
            let uses = self.instructions[i].uses();
            for op in uses.iter() {
                if let Operand::Var(s) = op {
                    self.ensure_variable_exists(&s);
                } else if let Operand::Temp(s) = op {
                    self.ensure_variable_exists(&s);
                }
            }
            let out = self.inst_liveness[i].live_out.clone();

            // handle callstart
            if let Instruction::CallStart(_) = &self.instructions[i] {
                let mut ops = Vec::new();
                for s in &out {
                    if s == "_ret" { continue; }
                    ops.push(Operand::Var(s.to_string()));
                }
                self.callstarts.push(ops);
            }

            if let Some(op) = self.instructions[i].def() {
                let def_name = op.print();
                self.ensure_variable_exists(&def_name);
                for out_var in out {
                    self.add_edge(&def_name, &out_var);
                }
            } 
        }
    }
    fn add_edge(&mut self, src: &String, dest: &String) {
        if src == dest { return; }
        self.interference_graph.variables.entry(src.to_string()).or_insert_with(|| 
            Variable { name:src.to_string(), register_id: 0, spilled: false });
        self.interference_graph.variables.entry(dest.to_string()).or_insert_with(|| 
            Variable { name:dest.to_string(), register_id: 0, spilled: false });

        self.interference_graph.edges.entry(src.to_string())
        .or_default().insert(dest.to_string());
        self.interference_graph.edges.entry(dest.to_string())
        .or_default().insert(src.to_string());   
    }

    fn ensure_variable_exists(&mut self, name: &str) {
        let key = name.to_string();
        self.interference_graph.variables.entry(key.clone()).or_insert_with(|| 
            Variable { name:name.to_string(), register_id: 0, spilled: false });
        self.interference_graph.edges.entry(key).or_insert_with(IndexSet::new);
    }


}