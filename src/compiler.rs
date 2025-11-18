use std::{cmp::Ordering, collections::HashMap, fs, time::Instant};

use crate::{intermediate::{analyzer::{SemanticAnalyzer, new_analyzer}, frame::Frame, instruction::Instruction, irgen::{CodeGen, new_codegen}}, optimizer::{cfg::{ControlFlowGraph, create_cfgs}, liveness::new_liveness_analyzer}, parser::{node::{NODE_COUNT, ParserNode}, parser::{Parser, new_parser}}};


static CALCULATE_TIME: bool = true;

static SOURCE_PATH: &str = "code.c";
static SOURCE_FILE_INFO: bool = false;

static PARSE_INFO: bool = false;

static TAC_PATH: &str = "tac.txt";
static MAKE_TAC_FILE: bool = true;

static PRINT_BLOCKS: bool = false;
static CFG_INFO: bool = false;

pub struct Compiler {
    input: String,
    program_node: ParserNode,
    instructions: Vec<Instruction>,
    frames: HashMap<String, Frame>,
    cfgs: Vec<ControlFlowGraph>,


}

pub fn new_compiler() -> Compiler {
    Compiler {
        input: String::new(), 
        program_node: ParserNode::Block(Vec::new()),
        instructions: Vec::new(),
        frames: HashMap::new(),
        cfgs: Vec::new(),
      }
}

impl Compiler {
    pub fn compile(&mut self, file_path: &str) {
        let now = Instant::now();
        
        self.parse(file_path);
        self.analyse_semantic();
        self.generate_ir();
        self.generate_cfgs();
        self.generate_assembly();

        if CALCULATE_TIME {
            let end_time = now.elapsed();
            println!("\nprogram duration: {} ms", end_time.as_millis());
        }
    }

    fn parse(&mut self, file_path: &str) {
        println!("\n-------------------- SOURCE CODE -------------------");
        println!("Reading file '{file_path}'");
        let input = fs::read_to_string(file_path).expect("file not found");
        if PARSE_INFO {
        println!("- '{file_path}'\nread: {} characters, {} lines", input.len(), input.lines().count());
        }
        println!("\n---------------------- PARSING ---------------------");
        println!("Starting Syntax Analysis:");
        let mut parser = new_parser(input.as_str()).unwrap();
        println!("- Parser created");
        let program_node = match parser.parse() {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };
        program_node.to_string();
        self.program_node = program_node;
        println!("- Abstract Syntax Tree created");
        if PARSE_INFO { 
            println!("node count: {}", NODE_COUNT.load(std::sync::atomic::Ordering::Relaxed))
        }
    }

    fn analyse_semantic(&mut self) {
        println!("\n---------------------- ANALYSIS --------------------");
        println!("Starting Semantic Analysis:");
        let mut analyzer = new_analyzer();
        println!("- Analyzer created");
        let res = analyzer.analyze(&mut self.program_node);
        match res {
            Err(e) => {
                println!("- Error: {}", e);
            },
            _ => println!("analyzer response: valid code"),
            
        }
        self.frames = analyzer.function_frames;
    }

    fn generate_ir(&mut self) {
        println!("\n------------------------ IR ------------------------");
        println!("Starting Intermediate Representation Generation:");
        let mut code_gen = new_codegen(self.frames.clone());
        println!("- Three Adress Code Gen created");
        code_gen.cgen(&self.program_node);

        if MAKE_TAC_FILE {
            fs::write(TAC_PATH, code_gen.print_instructions()).expect("write file failed");
            println!("- TAC file created at '{TAC_PATH}'");
        }
        
        self.instructions = code_gen.instructions;
        println!("- Instruction List created\ninstruction count: {}", self.instructions.len());
        let frames = code_gen.frames;
        println!("- Function Frames created\nframe count: {}", frames.len());
    }

    fn generate_cfgs(&mut self) {
        println!("\n---------------- CONTROL FLOW GRAPH ----------------");
        println!("Starting Block Building");
        self.cfgs = create_cfgs(&self.frames, &self.instructions);
        println!("- Control Flow Graphs created");
    
    
        let mut liveness_analyzer = new_liveness_analyzer(self.instructions.clone(), self.cfgs[0].blocks.clone());
        println!("- Liveness Analyzer created");
        liveness_analyzer.gen_live_out();
        
        if PRINT_BLOCKS {
            println!("BLOCKS:");
            println!("{:^9} {:^9}     {:^9}  {:^7}", "[id]", "[range]", "[edges]", "[label]");
            for block in &liveness_analyzer.blocks {
                println!("{}", block);
            }
        }
        if CFG_INFO {
            let mut block_count = 0;
            for cfg in &self.cfgs {
                block_count += cfg.blocks.len();
            }
            println!("block count: {}", block_count);
        }
        liveness_analyzer.gen_inst_live_out();    
        liveness_analyzer.create_interference_graph();
        if CFG_INFO {
            for (i, var) in liveness_analyzer.variables.iter().enumerate() {
                println!("{i} -> {:?}", var);
            } 
        }
    }

    fn generate_assembly(&mut self) {
        println!("\n---------------------- CODEGEN ---------------------");
        println!("Starting Register Allocation:");
        //let mut allocator = new_allocator(liveness_analyzer.variables);
        //println!("- Allocator created");
    }

}