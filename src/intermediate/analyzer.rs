use core::fmt;
use std::{collections::HashMap};
use crate::intermediate::frame::{Frame, new_frame};
use crate::parser::node::{ConstValue, ParserNode};
use crate::parser::token::Type;

static DEBUG_ANALYZER: bool = false;

pub struct SemanticAnalyzer {
    symbol_table: Vec<HashMap<String, Symbol>>,
    pub function_frames: HashMap<String, Frame>,
    current_frame: Option<Frame>,
    scope_count: usize,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope: usize,
    pub offset: i32,
    pub stype: Type,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable { initialized: bool },
    Function { args_size: usize },
}
#[derive(Debug)]
pub enum AnalyzerError {
    UndeclaredVar{var: String, last_func: String},
    InvalidNode(String),
    ScopeError(String),
    AlreadyDeclared(String),
    InvalidArguments(String),
    TypeMismatch{type1: Type, type2: Type, last_func: String},
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzerError::UndeclaredVar { var, last_func }
                => write!(f, "AnalyzerError: undeclared var '{}' found at {}", var, last_func),
            AnalyzerError::TypeMismatch {type1, type2, last_func }
                => write!(f, "AnalyzerError: type mismatch: '{}' and '{}' found at {}", type1.to_string(), type2.to_string(), last_func.to_string()),
            AnalyzerError::InvalidNode(s) | AnalyzerError::ScopeError(s)  
                => write!(f, "AnalyzerError: {}", s),
            AnalyzerError::InvalidArguments(s)
                => write!(f, "AnalyzerError: {}", s),
            AnalyzerError::AlreadyDeclared(s)
                => write!(f, "AnalyzerError: {}", s),
        }
    }
}


pub fn new_analyzer() -> SemanticAnalyzer {
    SemanticAnalyzer {
        symbol_table: vec!(HashMap::new()),
        function_frames: HashMap::new(),
        current_frame: None,
        scope_count: 0,
    }
}

impl SemanticAnalyzer {
    pub fn analyze(&mut self, program_node: &mut ParserNode) -> Result<(), AnalyzerError> {
        self.analyze_node(program_node)?;
        if self.current_frame.is_some() {
            let frame = self.current_frame.take().unwrap();
            self.function_frames.insert(frame.name.clone(), frame);
        }
        Ok(())
    }

    fn analyze_node(&mut self, node: &mut ParserNode) -> Result<Type, AnalyzerError> {
        match node {
            ParserNode::Block(nodes ) => {
                
                for n in nodes {
                    self.analyze_node(n)?;
                }

                
                self.symbol_table.pop().unwrap();
                if self.scope_count != 0 { self.scope_count -= 1}
            },
            ParserNode::FuncDecl { ident, args, block, ntype} => {
                
                let name = self.get_ident(ident)?;
                if self.scope_count != 0 {
                    return Err(AnalyzerError::InvalidNode(format!("'{}' decl inside block", name)))
                }


                if self.current_frame.is_some() {
                    let frame = self.current_frame.take().unwrap();
                    self.function_frames.insert(frame.name.clone(), frame);
                }

                if DEBUG_ANALYZER { println!("DEBUG_ANALYZER: new function frame: {}", name.clone())}

                self.current_frame = Some(new_frame(name.clone()));
                self.declare_function(&name, args.len(), ntype.clone())?;
                self.new_scope();

                for arg in args {
                    if let ParserNode::Var { ident, ntype } = arg {
                        self.declare_param(ident, true, *ntype)?;
                    }
                }
                self.analyze_node(block)?;
            },
            ParserNode::Declare { ident, exp, ntype } => {
                let name = self.get_ident(ident)?;
                let decl_type = *ntype;
                match ident.as_mut() {
                    ParserNode::Var {ntype , .. } => {
                        *ntype = decl_type;
                    },
                    _ => return Err(AnalyzerError::InvalidNode(format!(""))),
                }
                match exp {
                    Some(n) => {
                        let type2 = self.analyze_node(n)?;
                        self.expect_type(*ntype, type2)?;
                        self.declare_variable(&name, true, *ntype)?;
                    },
                    None => self.declare_variable(&name, false, *ntype)?,
                }
                self.debug_print();
                return Ok(*ntype)
            },
            ParserNode::Assign { left, right } => {
                let name = self.get_ident(left)?;

                if !self.is_declared(&name)? {
                    
                    return Err(AnalyzerError::UndeclaredVar{ var:name, last_func: self.frame_string()});
                }
                let type1 = self.initialize_variable(&name)?;
                let type2 = self.analyze_node(right)?;
                self.expect_type(type1, type2)?;
                return Ok(type1);
            },
            ParserNode::For { exp1, exp2, exp3, block } => {
                self.analyze_node(exp1)?;
                self.analyze_node(exp2)?;
                self.analyze_node(exp3)?;
                self.new_scope();
                self.analyze_node(block)?;
            },
            ParserNode::While { cond, block } => {
                self.analyze_node(cond)?;
                self.new_scope();
                self.analyze_node(block)?;
            }

            ParserNode::If { cond, block , else_stmt} => {
                self.analyze_node(cond)?;
                self.new_scope();
                self.analyze_node(block)?;
                match else_stmt {
                    Some(n) => {
                        if let ParserNode::If { .. } = **n {
                            self.analyze_node(n)?;
                        } else {
                            self.new_scope();
                            self.analyze_node(n)?;  
                        }
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
                let type1 = self.analyze_node(left)?;
                let type2 = self.analyze_node(right)?;
                self.expect_type(type1, type2)?;
                return Ok(type1);
            },

            ParserNode::Neg { val } | ParserNode::Complement { val } |
            ParserNode::Not { val } | ParserNode::SubExp { val } => {
                return self.analyze_node(val);
            },

            ParserNode::FuncCall { ident, args } => {
                let mut _ntype = Type::Void;
                match self.get_symbol(&ident) {
                    
                    Some(s) => {
                        _ntype = s.stype;
                        match s.kind {
                            SymbolKind::Function { args_size } => {
                                if args.len() != args_size {
                                    return Err(AnalyzerError::InvalidArguments("argument count invalid".into()));
                                }
                            }
                            _ => return Err(AnalyzerError::InvalidNode(ident.clone())),

                        }
                    }
                    None => return Err(AnalyzerError::UndeclaredVar{ var:ident.clone(), last_func: self.frame_string()}),
                }

                return Ok(_ntype);
            }

            ParserNode::Const(val) => {
                match val {
                    ConstValue::Int(_) => return Ok(Type::Int),
                    ConstValue::Float(_) => return Ok(Type::Float),
                    ConstValue::Double(_) => return Ok(Type::Double),
                    ConstValue::Char(_) => return Ok(Type::Char),
                    ConstValue::Void => return Ok(Type::Void),
                }
            },
            ParserNode::Var {ident, ..} => {
                self.is_initialized(&ident)?;
                return self.initialize_variable(ident);
            },
            

           /*  _ => return Err(AnalyzerError::InvalidNode("unknown node".to_string())) */
        }
        Ok(Type::Void)
    }

    fn declare_variable(&mut self, name: &String, initialized: bool, ntype: Type) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared(format!("'{}' already exists.", name)));
        }
        let scope = self.scope_count;
        if let Some(frame) = &mut self.current_frame {
            frame.allocate_local(name.clone(), self.scope_count,ntype);
        }
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable { initialized: initialized }, 
                scope,
                offset: 0,
                stype: ntype,
            },
        );
        Ok(())
    }
    fn declare_param(&mut self, name: &String, initialized: bool, ntype: Type) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("variable already exists".into()));
        }
        let scope = self.scope_count;
        if let Some(frame) = &mut self.current_frame {
            frame.allocate_param(name.clone(), self.scope_count,ntype);
        }
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable { initialized: initialized }, 
                scope,
                offset: 0,
                stype: ntype,
            },
        );
        self.debug_print();
        Ok(())
    }

    fn initialize_variable(&mut self, name: &String) -> Result<Type, AnalyzerError> {
        for t in self.symbol_table.iter_mut() {
            match t.get_mut(name) {
            Some(s) => {
                if let SymbolKind::Variable { initialized } = &mut s.kind {
                    *initialized = true;
                    return Ok(s.stype);
                } else {
                    return Err(AnalyzerError::InvalidNode("not a variable".into()))
                }
            },
            None => (),
        }
        }
        
        Err(AnalyzerError::UndeclaredVar{ var:name.clone(), last_func: self.frame_string()})
    }

    fn declare_function(&mut self, name: &String, args_size: usize, ntype: Type) -> Result<(), AnalyzerError> {
        if self.is_declared(name)? {
            return Err(AnalyzerError::AlreadyDeclared("function already exists".into()));
        }
        let scope = self.scope_count;
        self.current_table()?.insert(
            name.clone(), 
            Symbol {
                name: name.clone(),
                kind: SymbolKind::Function { args_size: args_size },
                scope: scope,
                offset: 0, 
                stype: ntype, },
                
            );
        self.debug_print();
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
        Err(AnalyzerError::UndeclaredVar{ var:var.clone(), last_func: self.frame_string()})
        
    }

    pub fn get_ident(&self, ident: &ParserNode) -> Result<String, AnalyzerError> {
        match ident {
            ParserNode::Var{ ident, ntype:_} => Ok(ident.clone()),
            _ => return Err(AnalyzerError::InvalidNode("left expression must be a variable".into()))
        }   
    }


    fn new_scope(&mut self) {
        self.scope_count += 1;
        self.symbol_table.push(HashMap::new());
    }

    fn expect_type(&mut self, type1: Type, type2: Type) -> Result<(), AnalyzerError> {
        if type1 == type2 {
            Ok(())
        } else {
            Err(AnalyzerError::TypeMismatch{type1, type2, last_func: self.frame_string()})
        }  
    }

    fn debug_print(&self) {
        if DEBUG_ANALYZER {
            let mut symbol_table_string = String::new();
            for t in self.symbol_table.iter() {
                symbol_table_string.push('(');
                for (i, s) in t.keys().into_iter().enumerate() {
                    symbol_table_string.push_str(format!("{}, ", s).as_str());
                    if i == t.len() - 1 {
                        symbol_table_string.pop();
                        symbol_table_string.pop();
                    }
                }
                symbol_table_string.push_str(") ");
            }
            println!("DEBUG_ANALYZER:{:^10} - ST({}): {:<50}",
            self.frame_string(),
            self.symbol_table.len(),
            symbol_table_string);
        }
        
    }

    pub fn frame_string(&self) -> String {
        match &self.current_frame {
            Some(frame) => frame.name.clone(),
            None => String::from("_global"),
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::parser::parser::new_parser;

    use super::*;

    #[test]
    fn analyzer_declare() {
        let cases = [
            "int x;", 
            "int y = 2;",
            "int main() { return 1 > 2; int x = 0; x=2; if(x*x ==2) {int y = 0; y = y + 2;} else { x = x - 1; } return x;}",
            "int func() { return 10; } int main() { return func(); }",
            "int func() { int x = 2; return x + 10; } int main() { int y = 9; return 9 * func(); }",
            "int func(int a, int b) {int z = a / b + 2;return z - 1;}int main() {int x = 0;int y = 10;int s = 25;if (x + 5 / 3) {y = y + 2;x = y * 5;
            s = 200;} else if (y < 2) {x = 10000 % y << s;} else {y = 999;}func(x-2, 2);}"
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
            "int a=1;int b=1; int c=1; int d=1; int e=1; int f=1; int g=1; int h=1; int i=1; int j=1;a + b * c - d / e & f | g ^ h << 2 >> 1 && i || j",
            "int a=1;int b=a; int c=1; a + b * c",
            "int a=1;int b=1; int c=1; int d=1;a - b / c % d",
            "int a=1;int b=1; int c=1; int d=1; int e=1;a & b | c ^ d << e",
            "int a=1;int b=1; int c=1;(a + b) * c",
            "int a=1;int b=1; int c=1;!(a + b * ~c)",
            "int a=1;int b=1; int c=1;!a + b * ~c",
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