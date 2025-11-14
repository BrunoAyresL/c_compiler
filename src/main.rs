use std::{fs};
use crate::{
    codegen::allocation::new_allocator, 
    intermediate::{analyzer::new_analyzer, cfg::create_cfgs, irgen::new_codegen},
    parser::{node::NODE_COUNT, parser::new_parser}
};

use std::sync::atomic::Ordering;

mod parser;
mod intermediate;
mod codegen;

static SOURCE_PATH: &str = "code.c";
static TAC_PATH: &str = "tac.txt";

fn main() {
    println!("\n-------------------- SOURCE CODE -------------------");
    println!("Reading file '{SOURCE_PATH}'");
    let input = fs::read_to_string(SOURCE_PATH).expect("file not found");
    println!("- '{SOURCE_PATH}'\nread: {} characters, {} lines", input.len(), input.lines().count());
    
    println!("---------------------- PARSING ---------------------");
    println!("Starting Syntax Analysis:");
    let mut parser = new_parser(input.as_str()).unwrap();
    println!("- Parser created");
    let mut program_node = match parser.parse() {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    program_node.to_string();
    println!("- AST created\nnode count: {}", NODE_COUNT.load(Ordering::Relaxed));

    println!("---------------------- ANALYSIS --------------------");
    println!("Starting Semantic Analysis:");
    let mut analyzer = new_analyzer();
    println!("- Analyzer created");
    let res = analyzer.analyze(&mut program_node);
    match res {
        Err(e) => {
            println!("- Error: {}", e);
            //analyzer.print();
        },
        _ => println!("- Analyzer response: valid code"),
        
    }
    println!("------------------------ IR ------------------------");
    println!("Starting Intermediate Representation Generation:");
    let mut code_gen = new_codegen(analyzer.function_frames);
    println!("- TAC Code Gen created");
    code_gen.cgen(&program_node);

    fs::write(TAC_PATH, code_gen.print_instructions()).expect("write file failed");
    println!("- TAC file created at '{TAC_PATH}'");

    let instructions = code_gen.instructions;
    println!("- Instruction List created\ncount: {}", instructions.len());
    let frames = code_gen.frames;
    println!("- Function Frames created\ncount: {}", frames.len());
    
    println!("---------------- CONTROL FLOW GRAPH ----------------");
    println!("Starting Block Building");
    let cfgs = create_cfgs(&frames, &instructions);
    for block in &cfgs[0].blocks {
        println!("BLOCK\t->\t{}", block);
    }

    println!("---------------------- CODEGEN ---------------------");
    println!("Starting Register Allocation:");
    let mut allocator = new_allocator(instructions, frames);
    println!("- Allocator created");
    allocator.get_liveness();    
    allocator.allocate_registers();
    //for l in allocator.live_ranges {
        //println!("{:?}", l);
    //}

    println!("exit");
}

