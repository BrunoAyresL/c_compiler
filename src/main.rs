use std::{fs};
use crate::{allocation::new_allocator, analyzer::new_analyzer, irgen::new_codegen, parser::new_parser};

pub mod error;

pub mod token;
pub mod lexer;

pub mod node;
pub mod parser;

pub mod frame;
pub mod analyzer;

pub mod instruction;
pub mod irgen;

pub mod allocation;

fn main() {
    println!("\n-Start-");
    let input = fs::read_to_string(&"code.c")
        .expect("file not found");

    //println!("code.c input:\n{input}");

    let mut parser = new_parser(input.as_str()).unwrap();
    println!("parsing...");
    let mut program_node = match parser.parse() {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    println!("parsing done");
    //println!("{}", program_node.to_string());

    println!("analyzing...");
    let mut analyzer = new_analyzer();
    let res = analyzer.analyze(&mut program_node);
    match res {
        Err(e) => {
            println!("Err: {}", e);
            //analyzer.print();
        },
        _ => println!("analyzing done"),
        
    }

    let mut code_gen = new_codegen(analyzer.function_frames);
    code_gen.cgen(&program_node);
    fs::write("tac.txt", code_gen.print_instructions()).expect("write file failed");

    let instructions = code_gen.instructions;
    let frames = code_gen.frames;
    let mut allocator = new_allocator(instructions, frames);
    let lv = allocator.get_liveness();
    for l in lv {
        println!("{:?}\n", l)
    }

    println!("-End-");
}

