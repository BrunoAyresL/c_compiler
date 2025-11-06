use std::{fs, process::Command};
use crate::{analyzer::new_analyzer, irgen::new_codegen, parser::new_parser};

pub mod lexer;
pub mod parser;
pub mod token;
pub mod node;
pub mod analyzer;
pub mod error;
pub mod symboltable;
pub mod irgen;
pub mod instruction;

fn main() {
    let input = fs::read_to_string(&"code.c")
        .expect("file not found");

    //println!("code.c input:\n{input}");

    let mut parser = new_parser(input.as_str()).unwrap();
    println!("parsing...");
    let mut program_node = match parser.parse() {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    //println!("\nresult:\n");
    //println!("{}", program_node.to_string());

    println!("analyzing...");
    let mut analyzer = new_analyzer();
    let res = analyzer.analyze(&mut program_node);
    match res {
        Err(e) => {
            println!("Erro: {:?}", e);
            //analyzer.print();
        },
        _ => println!("program is valid"),
        
    }
    println!("\nCode Gen - TAC:");
    let mut code_gen = new_codegen();
    code_gen.cgen(&program_node);
    print!("{}", code_gen.print_instructions());
    println!("\n-End-");
    /*
    let mut code_gen = new_code_generator(analyzer.complete_table);
    
    println!("assembly:\n{}", code_gen.gen_assembly(&program_node));
    fs::write("output.s", code_gen.gen_assembly(&program_node)).expect("can't create file");
    Command::new("gcc")
        .arg("output.s")
        .arg("-o")
        .arg("out")
        .spawn()
        .expect("Failed");
     */
     
}

