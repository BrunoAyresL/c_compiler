use std::collections::HashMap;
use crate::node::ParserNode;

pub struct SemanticAnalyzer {
    symbol_table: Vec<HashMap<String, Symbol>>,
    scope_count: usize,
}

struct Symbol {
    name: String,
    scope: usize,
    kind: SymbolKind,
}

enum SymbolKind {
    Variable {  },
    Function { params: Vec<Symbol> },
}
#[derive(Debug)]
pub enum AnalyzerError {
    UndeclaredVar(String),
    InvalidNode(String),
    ScopeError(String),
    AlreadyDeclared(String),
}

pub fn new_analyzer() -> SemanticAnalyzer {
    SemanticAnalyzer {
        symbol_table: vec!(HashMap::new()),
        scope_count: 0,
    }
}

impl SemanticAnalyzer {
    pub fn analyze(&mut self, program_node: ParserNode) -> Result<(), AnalyzerError>{
        self.analyze_node(program_node)
    }

    fn analyze_node(&mut self, node: ParserNode) -> Result<(), AnalyzerError> {
        match node {
            ParserNode::Block(nodes ) => {
                self.scope_count += 1;
                self.symbol_table.push(HashMap::new());

                for n in nodes {
                    self.analyze_node(n)?;
                }

                self.scope_count -= 1;
                self.symbol_table.pop();
            },
            ParserNode::FuncDecl { ident, block } => {
                if self.scope_count != 0 {
                    return Err(AnalyzerError::InvalidNode("function declaration inside block".to_string()))
                }
                let name = match *ident {
                    ParserNode::Var(id) => id,
                    _ => return Err(AnalyzerError::InvalidNode("function declaration must have ident".into()))
                };
                self.declare_function(&name)?;

                self.analyze_node(*block)?

            },
            ParserNode::Declare { ident, exp } => {
                let name = match *ident {
                    ParserNode::Var(id) => id,
                    _ => return Err(AnalyzerError::InvalidNode("declaration must have ident".into()))
                };
                self.declare_variable(&name)?;
            },
            _ => return Err(AnalyzerError::InvalidNode("unknown node".to_string()))
        }
        Ok(())
    }

    fn declare_variable(&mut self, name: &String) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("variable already exists".into()));
        }

        let scope = self.scope_count;
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                name: name.to_string(), 
                kind: SymbolKind::Variable {  }, 
                scope: scope
            },
        );
        Ok(())
    }

    fn declare_function(&mut self, name: &String) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("function already exists".into()));
        }

        let scope = self.scope_count;
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                name: name.to_string(), 
                kind: SymbolKind::Function { params: Vec::new() }, 
                scope: scope},
            );
        Ok(())
    }

    fn current_table(&mut self) -> Result<&mut HashMap<String, Symbol>, AnalyzerError> {
        match self.symbol_table.last_mut() {
            Some(h) => Ok(h),
            None => Err(AnalyzerError::ScopeError("last table not found".into())),
        }
    }

    fn is_declared(&mut self, var: &String) -> Result<bool, AnalyzerError> {
        if self.current_table()?.contains_key(var) {
            return Ok(true)
        } else {
            return Ok(false)
        }
    }


    pub fn print(&self) {
        for (i, t) in self.symbol_table.iter().enumerate() {
            print!("\nSCOPE {}:\n\t", i);
            for s in t.keys() {
                print!("{} ", s);
            }
        }
    }

}
