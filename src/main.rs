mod optimizer;
mod parser;
mod intermediate;
mod codegen;
mod compiler;

use crate::compiler::new_compiler;

fn main() {
    let mut compiler = new_compiler();
    compiler.compile("code.c");

}

