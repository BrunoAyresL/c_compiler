// Live Range 
// Interference
// Node, Edge

use std::{any::type_name, collections::{HashMap, HashSet}};

use crate::optimizer::liveness::{InterferenceGraph, Variable};


static REG_COUNT: usize = 8;

pub struct Allocator {
    register_count: usize,
    pub ifr_graph: InterferenceGraph,
    pub spill: HashMap<String, i32>,
}

pub fn new_allocator(ifr_graph: InterferenceGraph) -> Allocator {
    Allocator { 
        register_count: REG_COUNT,
        ifr_graph,
        spill: HashMap::new(),
    }
}


impl Allocator {

    fn variables(&self) -> & HashMap<String, Variable> {
        &self.ifr_graph.variables
    }
    fn edges(&self) -> & HashMap<String, HashSet<String>> {
        &self.ifr_graph.edges
    }

    pub fn coloring(&mut self) {
        let mut stack  = Vec::new();
        let mut offset = 0;
        let reg_count = self.register_count;
        
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
        self.ifr_graph.edges = HashMap::new();
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
                let used_colors: HashSet<usize> = self.edges()[&name]
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

    fn add_var(&mut self, var: Variable, edges: HashSet<String>) {
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
        self.ifr_graph.variables.remove(&name);
        self.ifr_graph.edges.remove(&name);
        for (_, set) in self.ifr_graph.edges.iter_mut() {
            set.remove(&name);
        }
    }


}