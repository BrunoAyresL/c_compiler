use std::{fs, process::Command};
use crate::parser::new_parser;

pub mod lexer;
pub mod parser;
pub mod codegen;
pub mod token;
pub mod node;

fn main() {
    let input = fs::read_to_string(&"code.c")
        .expect("file not found");

    println!("code.c input:\n{input}");


    let mut parser = new_parser(input.as_str());
    println!("\nparsing...");
    let program_node = parser.parse();
    println!("result:\n");
    println!("{}", program_node.to_string());

    println!("assembly:\n{}", program_node.gen_assembly());
    fs::write("output.s", program_node.gen_assembly()).expect("can't create file");
    Command::new("gcc")
        .arg("output.s")
        .arg("-o")
        .arg("out")
        .spawn()
        .expect("Failed");

}

