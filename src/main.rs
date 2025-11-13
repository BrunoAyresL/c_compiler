use std::{fs};
use crate::{
    codegen::allocation::new_allocator, 
    intermediate::analyzer::new_analyzer,
    intermediate::irgen::new_codegen, 
    parser::parser::new_parser
};

mod parser;
mod intermediate;
mod codegen;


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
    println!("generating TAC...");
    let mut code_gen = new_codegen(analyzer.function_frames);
    code_gen.cgen(&program_node);
    fs::write("tac.txt", code_gen.print_instructions()).expect("write file failed");
    println!("TAC done");

    let instructions = code_gen.instructions;
    let frames = code_gen.frames;
    println!("allocating...");
    let mut allocator = new_allocator(instructions, frames);
    allocator.get_liveness();    
    allocator.allocate_registers();
    //for l in allocator.live_ranges {
        //println!("{:?}", l);
    //}
    println!("allocation done");

    println!("-End-");
}

