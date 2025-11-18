
use crate::compiler::new_compiler;


mod optimizer;
mod parser;
mod intermediate;
mod codegen;
mod compiler;

// debug



fn main() {

    let mut compiler = new_compiler();
    compiler.compile("code.c");



    


    
}

