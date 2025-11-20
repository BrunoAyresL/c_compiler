// Live Range 
// Interference
// Node, Edge

use indexmap::{IndexMap, IndexSet};
use crate::{intermediate::frame::{Frame}, optimizer::liveness::{InterferenceGraph, Variable}};


static REG_COUNT: usize = 12;

pub struct Allocator {
    register_count: usize,
    pub ifr_graph: InterferenceGraph,
    pub frame: Frame,
    pub spill: IndexMap<String, i32>,
}

pub fn new_allocator(ifr_graph: InterferenceGraph, frame: Frame) -> Allocator {
    Allocator { 
        register_count: REG_COUNT,
        frame,
        ifr_graph,
        spill: IndexMap::new(),
    }
}


impl Allocator {

    fn variables(&self) -> & IndexMap<String, Variable> {
        &self.ifr_graph.variables
    }
    fn edges(&self) -> & IndexMap<String, IndexSet<String>> {
        &self.ifr_graph.edges
    }

    pub fn coloring(&mut self) {
        let mut stack  = Vec::new();
        let mut offset = 0;
        let reg_count = self.register_count;
        
        // pre-coloring
        let mut pre_color = 6;
        for var in &self.frame.params {
            if self.variables().contains_key(&var.name) { 
                let v = self.ifr_graph.variables.get_mut(&var.name).unwrap();
                v.register_id = pre_color;
                pre_color += 1;
            }
        }


        while !self.variables().is_empty() {
            let mut to_remove: Option<String> = None;
            for (name, var) in self.ifr_graph.variables.iter() {
                let edges_len = self.edges()[name].len();
                
                if edges_len < reg_count {
                    to_remove = Some(name.clone());
                    stack.push((var.clone(), self.edges()[name].clone(), false));
                    break;
                }  
            }

            if let Some(name) = to_remove {
                self.remove_var(name);
                
                continue;
            }
            
            let (name, var) = self.variables().iter().last().unwrap();
            stack.push((var.clone(), self.edges()[name].clone(), true));
            let name = name.clone();
            self.remove_var(name);
        }
        self.ifr_graph.edges = IndexMap::new();
        // coloring
        while !stack.is_empty() {
            let (mut var, edges, is_spilled) = stack.pop().expect("invalid pop stack on coloring");
            if is_spilled {
                var.spilled = true;
                self.spill.insert(var.name.clone(), offset);
                offset -= 8;
            } else {
                let name = var.name.clone();
                let mut reg_id = 0;
                self.add_var(var.clone(), edges.clone());
                if self.variables().get(&name).unwrap().register_id != 0 {
                    continue;
                }

                let used_colors: IndexSet<usize> = self.edges()[&name]
                .iter()
                .filter_map(|other| self.variables().get(other))
                .map(|v| v.register_id)
                .collect();
            
                while used_colors.contains(&reg_id) {
                    reg_id += 1;
                }
                self.ifr_graph.variables.get_mut(&name).unwrap().register_id = reg_id;

            }
        }


        
    }

    fn add_var(&mut self, var: Variable, edges: IndexSet<String>) {
        let name = var.name.clone();
        self.ifr_graph.variables.insert(name.clone(), var.clone());
        self.ifr_graph.edges.insert(name.clone(), edges.clone());
        
        for other in &edges {
            if other == &name {
                continue;
            }

            self.ifr_graph.edges.entry(other.clone()).or_default()
            .insert(name.clone());  
        }
    }

    fn remove_var(&mut self, name: String) {
        self.ifr_graph.variables.swap_remove(&name);
        self.ifr_graph.edges.swap_remove(&name);
        for (_, set) in self.ifr_graph.edges.iter_mut() {
            set.swap_remove(&name);
        }
    }


}