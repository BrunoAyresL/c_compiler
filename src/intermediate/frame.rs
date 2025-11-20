use crate::{intermediate::analyzer::{Symbol, SymbolKind}, parser::token::Type};

#[derive(Clone, Debug)]
pub struct Frame {
    pub name: String,
    pub params: Vec<Symbol>,
    locals: Vec<Symbol>,
    pub params_size: usize,
    pub locals_size: usize,
    pub range: (usize, usize),
}

pub fn new_frame(name: String) -> Frame {
    Frame { 
        name, 
         params: Vec::new(), 
        locals: Vec::new(),
        params_size: 0,
        locals_size: 0,
        range: (0,0)
    }
}

impl Frame {
    pub fn allocate_local(&mut self, name: String, scope: usize, stype: Type) -> &Symbol {
        let local = Symbol {
            name,
            kind: SymbolKind::Variable { initialized: true },
            scope,
            offset:  -((self.locals_size + stype.size()) as i32),
            stype,
        };
        self.locals.push(local);
        self.locals_size += stype.size();
        &self.locals.last().unwrap()
    }

    pub fn allocate_param(&mut self, name: String, scope: usize, stype: Type) -> &Symbol {
        let param = Symbol {
            name,
            kind: SymbolKind::Variable { initialized: true },
            scope,
            offset:  ((self.locals_size + stype.size()) as i32),
            stype,
        };
        self.params.push(param);
        self.params_size += stype.size();
        &self.params.last().unwrap()
    }
}