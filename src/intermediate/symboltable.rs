use std::collections::HashMap;

use crate::analyzer::{AnalyzerError, Symbol, SymbolKind};
#[derive(Debug)]
pub struct SymbolTable {
    pub table: HashMap<String, Symbol>,
    pub scope: usize,
}

pub fn new_symbol_table(table: HashMap<String, Symbol>, scope: usize) -> SymbolTable {
    SymbolTable { table, scope }
}

impl SymbolTable {
    pub fn look_up(&self, name: &String) -> Result<bool, AnalyzerError> {
        Ok(self.table.contains_key(name))
    }

    pub fn get_var_offset(&self, name: &String) -> Result<usize, AnalyzerError> {
       match self.table.get(name) {
        Some(symbol) => Ok(symbol.offset),
        None => Ok(0)
       }
    }

    pub fn var_count(&self) -> Result<usize, AnalyzerError> {
        let mut count = 0;
        for symbol in self.table.values() {
            if matches!(symbol.kind, SymbolKind::Variable { initialized: _ }) {
                count += 1;
            }
        }
        Ok(count)
    }
}