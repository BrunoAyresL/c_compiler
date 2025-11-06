use std::env::var;
use std::{collections::HashMap};
use crate::node::ParserNode;
use crate::symboltable::{SymbolTable, new_symbol_table};

pub struct SemanticAnalyzer {
    symbol_table: Vec<HashMap<String, Symbol>>,
    scope_count: usize,
    var_count: usize,
    pub complete_table: Vec<SymbolTable>,
    
}

#[derive(Debug)]
pub struct Symbol {
   pub kind: SymbolKind,
   pub offset: usize,
}

#[derive(Debug)]
pub enum SymbolKind {
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
    TableNotFound(String),
}

pub fn new_analyzer() -> SemanticAnalyzer {
    SemanticAnalyzer {
        symbol_table: vec!(HashMap::new()),
        scope_count: 0,
        var_count: 0,
        complete_table: Vec::new(),
    }
}

impl SemanticAnalyzer {
    pub fn analyze(&mut self, program_node: &mut ParserNode) -> Result<(), AnalyzerError> {
        // self.print();
        self.analyze_node(program_node)?;
        
        Ok(())
    }

    fn analyze_node(&mut self, node: &mut ParserNode) -> Result<(), AnalyzerError> {
        match node {
            ParserNode::Block(nodes ) => {
                
                for n in nodes {
                    self.analyze_node(n)?;
                }

                let table = self.symbol_table.pop().unwrap();
                self.complete_table.push(new_symbol_table(table, self.scope_count));
                if self.scope_count != 0 { self.scope_count -= 1}
            },
            ParserNode::FuncDecl { ident, args, block, size} => {
                if self.scope_count != 0 {
                    return Err(AnalyzerError::InvalidNode("function declaration inside block".to_string()))
                }
                let prev_var_count = self.var_count;
                let name = self.get_ident(ident)?;
                let args_size = args.len();

                self.declare_function(&name, args_size)?;

                self.new_scope();

                for arg in args {
                    self.declare_variable(&arg.to_string(), true)?;
                }
                
                self.analyze_node(block)?;
                let var_total = self.var_count - prev_var_count;
                *size = var_total * 4;
 
            },
            ParserNode::Declare { ident, exp } => {
                let name = self.get_ident(ident)?;

                match exp {
                    Some(n) => {
                        self.analyze_node(n)?;
                        self.declare_variable(&name, true)?;
                    },
                    None => self.declare_variable(&name, false)?,
                }
                
            },
            ParserNode::Assign { left, right } => {
                let name = self.get_ident(left)?;

                if !self.is_declared(&name)? {
                    
                    return Err(AnalyzerError::UndeclaredVar(name));
                }
                self.initialize_variable(&name)?;

                self.analyze_node(right)?;

            },
            ParserNode::If { cond, block , else_stmt} => {

                self.analyze_node(cond)?;
                self.new_scope();
                self.analyze_node(block)?;
                match else_stmt {
                    Some(n) => {
                        self.new_scope();
                        self.analyze_node(n)?
                    },
                    None => (),
                }
            },
            ParserNode::Return { exp } => {
                self.analyze_node(exp)?;
            },

            ParserNode::Expression(nodes) => {
                for n in nodes {
                    self.analyze_node(n)?;
                }
            },

            ParserNode::Add {left, right} | ParserNode::Sub {left, right} |
            ParserNode::Mul {left, right} | ParserNode::Div {left, right} |
            ParserNode::Mod {left, right} | ParserNode::ShiftLeft {left, right} |
            ParserNode::ShiftRight {left, right} | ParserNode::Greater {left, right} |
            ParserNode::GreaterEqual {left, right} | ParserNode::Less {left, right} | 
            ParserNode::LessEqual {left, right} | ParserNode::Equal {left, right} |
            ParserNode::NotEqual {left, right} | ParserNode::BitwiseAnd {left, right} |
            ParserNode::BitwiseXor {left, right} | ParserNode::BitwiseOr {left, right} |
            ParserNode::LogicalAnd {left, right} | ParserNode::LogicalOr {left, right} => {
                self.analyze_node(left)?;
                self.analyze_node(right)?;
            },

            ParserNode::Neg { val } | ParserNode::Complement { val } |
            ParserNode::Not { val } | ParserNode::SubExp { val } => {
                self.analyze_node(val)?;
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
                            _ => return Err(AnalyzerError::TypeMismatch(ident.clone())),

                        }
                    }
                    None => return Err(AnalyzerError::UndeclaredVar(format!("'{}' not found (func call)",ident.clone()))),
                }
            }

            ParserNode::Const(_) => {

            },
            ParserNode::Var(var) => {
                if self.scope_count > 0 {
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
        let localvar_count = self.get_localvar_count()?;
        self.current_table()?.insert(
            name.clone(), 
            Symbol { 
                kind: SymbolKind::Variable { initialized: initialized }, 
                offset: localvar_count * 8,
            },
        );
        self.var_count += 1;
        Ok(())
    }

    fn initialize_variable(&mut self, name: &String) -> Result<(), AnalyzerError> {
        for t in self.symbol_table.iter_mut() {
            match t.get_mut(name) {
            Some(s) => {
                if let SymbolKind::Variable { initialized } = &mut s.kind {
                    *initialized = true;
                    return Ok(());
                } else {
                    return Err(AnalyzerError::InvalidNode("not a variable".into()))
                }
            },
            None => (),
        }
        }
        
        Err(AnalyzerError::UndeclaredVar(format!("can't initialize undeclared variable '{}'", name.clone())))
    }

    fn declare_function(&mut self, name: &String, args_size: usize) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("function already exists".into()));
        }

        let localvar_count = self.get_localvar_count()?;
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                kind: SymbolKind::Function { args_size: args_size }, offset: localvar_count * 4 },
                
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

    pub fn get_ident(&self, ident: &ParserNode) -> Result<String, AnalyzerError> {
        match ident {
            ParserNode::Var(id) => Ok(id.clone()),
            _ => return Err(AnalyzerError::InvalidNode("left expression must be a variable".into()))
        }   
    }

    pub fn get_localvar_count(&self) -> Result<usize, AnalyzerError> {
        Ok(self.symbol_table.last().iter().len())
    }

    fn new_scope(&mut self) {
        self.scope_count += 1;
        self.symbol_table.push(HashMap::new());
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


#[cfg(test)]
mod tests {
    use crate::{parser::{new_parser}};

    use super::*;

    #[test]
    fn analyzer_declare() {
        let cases = [
            "int x;", 
            "int y = 2;",
            "int main() { return 1 > 2; int x = 0; x=2; if(x*x ==2) {int y = 0; y = y + 2;} else { x = x - 1; } return x;}",
            "int func() { return 10; } int main() { return func(); }",
            "int func() { int x = 2; return x + 10; } int main() { int y = 9; return 9 * func(); }"
        ];
        
        for input in cases {
            let mut analyzer = new_analyzer();
            let mut parser = new_parser(input).unwrap();
            let mut program_node = parser.parse().unwrap();

            let got = analyzer.analyze(&mut program_node);
            assert!(matches!(got, Ok(_)));
        }
    }

    #[test]
    fn analyzer_expression() { 
        let cases = [
            "5 + 3 == 2+1>>3*4",
            "a + b * c - d / e & f | g ^ h << 2 >> 1 && i || j",
            "a + b * c",
            "a - b / c % d",
            "a & b | c ^ d << e",
            "(a + b) * c",
            "!(a + b * ~c)",
            "!a + b * ~c",
        ];
        
        for input in cases {
            let mut analyzer = new_analyzer();

            let mut parser = new_parser(input).unwrap();
            let mut program_node = parser.parse().unwrap();

            let got = analyzer.analyze(&mut program_node);
            assert!(matches!(got, Ok(_)));
        }
    }
}