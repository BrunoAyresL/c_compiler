use std::{collections::HashMap, env::args};
use crate::node::ParserNode;

pub struct SemanticAnalyzer {
    symbol_table: Vec<HashMap<String, Symbol>>,
    scope_count: usize,
}

struct Symbol {
    scope: usize,
    kind: SymbolKind,
}

enum SymbolKind {
    Variable { initialized: bool },
    Function { args_size: usize },
}
#[derive(Debug)]
pub enum AnalyzerError {
    UndeclaredVar(String),
    InvalidNode(String),
    ScopeError(String),
    AlreadyDeclared(String),
    InvalidArguments(String),
    TypeMismatch(String),
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
            ParserNode::FuncDecl { ident, args, block } => {
                if self.scope_count != 1 {
                    return Err(AnalyzerError::InvalidNode("function declaration inside block".to_string()))
                }
                let name = self.get_ident(*ident)?;
                let args_size = args.len();
                for arg in args {
                    self.analyze_node(arg)?;
                    
                }
                self.declare_function(&name, args_size)?;

                self.analyze_node(*block)?

            },
            ParserNode::Declare { ident, exp } => {
                let name = self.get_ident(*ident)?;

                match exp {
                    Some(n) => {
                        self.analyze_node(*n)?;
                        self.declare_variable(&name, true)?;
                    },
                    None => self.declare_variable(&name, false)?,
                }
                
            },
            ParserNode::Assign { left, right } => {
                let name = self.get_ident(*left)?;

                if !self.is_declared(&name)? {
                    return Err(AnalyzerError::UndeclaredVar(name));
                }
                self.initialize_variable(&name)?;

                self.analyze_node(*right)?;

            },
            ParserNode::If { cond, block } => {

                self.analyze_node(*cond)?;
                self.analyze_node(*block)?;
            },
            ParserNode::Return { exp } => {
                self.analyze_node(*exp)?;
            },

            ParserNode::Add {left, right} | ParserNode::Sub {left, right} |
            ParserNode::Mul {left, right} | ParserNode::Div {left, right} |
            ParserNode::Mod {left, right} | ParserNode::ShiftLeft {left, right} |
            ParserNode::ShiftRight {left, right} | ParserNode::Greater {left, right} |
            ParserNode::GreaterEqual {left, right} | ParserNode::Less {left, right} | 
            ParserNode::LessEqual {left, right} | ParserNode::Equal {left, right} |
            ParserNode::NotEqual {left, right} | ParserNode::BitwiseAnd {left, right} |
            ParserNode::BitwiseXor {left, right} | ParserNode::BitwiseOr {left, right} => {
                self.analyze_node(*left)?;
                self.analyze_node(*right)?;
            },

            ParserNode::Neg { val } | ParserNode::Complement { val } |
            ParserNode::Not { val } | ParserNode::SubExp { val } => {
                self.analyze_node(*val)?;
            },

            ParserNode::FuncCall { ident, args } => {

                match self.get_symbol(&ident) {
                    Some(s) => {
                        match s.kind {
                            SymbolKind::Function { args_size } => {
                                if args.len() != args_size {
                                    return Err(AnalyzerError::InvalidArguments("argument count invalid".into()));
                                }
                            }
                            _ => return Err(AnalyzerError::TypeMismatch(ident)),

                        }
                    }
                    None => return Err(AnalyzerError::UndeclaredVar(ident)),
                }
            }

            ParserNode::Const(_) => {

            },
            ParserNode::Var(var) => {
                if self.scope_count > 1 {
                    self.is_initialized(&var)?;
                }
            },
            

            _ => return Err(AnalyzerError::InvalidNode("unknown node".to_string()))
        }
        Ok(())
    }

    fn declare_variable(&mut self, name: &String, initialized: bool) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("variable already exists".into()));
        }

        let scope = self.scope_count;
        self.current_table()?.insert(
            name.clone(), 
            Symbol { 
                kind: SymbolKind::Variable { initialized: initialized }, 
                scope: scope
            },
        );
        Ok(())
    }

    fn initialize_variable(&mut self, name: &String) -> Result<(), AnalyzerError> {
        match self.current_table()?.get_mut(name) {
            Some(s) => {
                if let SymbolKind::Variable { initialized } = &mut s.kind {
                    *initialized = true;
                } else {
                    return Err(AnalyzerError::InvalidNode("not a variable".into()))
                }
            },
            None => return Err(AnalyzerError::UndeclaredVar(name.clone()))
        }
        Ok(())
    }

    fn declare_function(&mut self, name: &String, args_size: usize) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("function already exists".into()));
        }

        let scope = self.scope_count;
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                kind: SymbolKind::Function { args_size: args_size }, 
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
        for (_, table) in self.symbol_table.iter().enumerate() {
            if table.contains_key(var) {
                return Ok(true)
            }
        }
        Ok(false)
    }
    fn get_symbol(&mut self, var: &String) -> Option<&Symbol> {
        for (_, table) in self.symbol_table.iter().enumerate() {
            match table.get(var) {
                Some(s) => return Some(s),
                None => (),
            }
        }
        None
    }

    fn is_initialized(&mut self, var: &String) -> Result<bool, AnalyzerError> {
        for (_, table) in self.symbol_table.iter().enumerate() {
            match table.get(var) {
                Some(s) => {
                    if let SymbolKind::Variable { initialized } = s.kind {
                        return Ok(initialized)
                    }
                },
                None => (),
            }    
        }
        Err(AnalyzerError::UndeclaredVar(var.clone()))
    }

    pub fn get_ident(&self, ident: ParserNode) -> Result<String, AnalyzerError> {
        match ident {
            ParserNode::Var(id) => Ok(id),
            _ => return Err(AnalyzerError::InvalidNode("left expression must be a variable".into()))
        }   
    }

    pub fn print(&self) {
        print!("Symbol Table:");
        for (_, t) in self.symbol_table.iter().enumerate() {
            print!("(");
            for s in t.keys() {
                print!(" {}", s);
            }
            print!(" )");
        }
    }

}
