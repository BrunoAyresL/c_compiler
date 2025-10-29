struct Analyzer {
    symbol_table: Vec<Variable>,
    scope_count: usize,
}

struct Variable {
    name: String,
    scope: usize,
    value: VarType,
    is_initialized: bool,
}

enum VarType {
    Int(usize),
    Float(f32),
    Double(f64),
    Char(char),
}


