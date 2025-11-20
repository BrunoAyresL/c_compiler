
use std::{ collections::HashMap, fs, io::{self, Write}, process::Command, thread::sleep, time::{Duration, Instant}};
use crate::{codegen::{allocation::new_allocator, codegen::new_asm_generator}, intermediate::{analyzer::new_analyzer, frame::Frame, instruction::Instruction, irgen::new_codegen}, optimizer::{cfg::{ControlFlowGraph, create_cfgs}, liveness::{InterferenceGraph, Variable, new_liveness_analyzer}}, parser::{node::{NODE_COUNT, ParserNode}, parser::{Parser, new_parser}}};


static CALCULATE_TIME: bool = true;

static PARSE_INFO: bool = false;

static TAC_PATH: &str = "tac.txt";
static MAKE_TAC_FILE: bool = true;

static PRINT_BLOCKS: bool = false;
static CFG_INFO: bool = false;

static CODEGEN_INFO: bool = true;
static MAKE_ASM_FILE: bool = true;
static ASM_PATH: &str = "code.s";
static ASM_CODE_OBJECT: &str = "code.o";
static HELPER_FILE: &str = "main.c";
static EXECUTABLE: &str = "exec.exe";

pub struct Compiler {
    program_node: ParserNode,
    instructions: Vec<Instruction>,
    frames: HashMap<String, Frame>,
    cfgs: Vec<ControlFlowGraph>,
    interference_graphs: Vec<InterferenceGraph>,
}

pub fn new_compiler() -> Compiler {
    Compiler {
        program_node: ParserNode::Block(Vec::new()),
        instructions: Vec::new(),
        frames: HashMap::new(),
        cfgs: Vec::new(),
        interference_graphs: Vec::new(),
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
            println!("\ncompile duration: {} ms", end_time.as_millis());
        }

        self.linker_run();
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
        self.cfgs = create_cfgs(&mut self.frames, &self.instructions);
        println!("- Control Flow Graphs created");
    
        for cfg in &self.cfgs {
            let mut liveness_analyzer = new_liveness_analyzer(self.instructions.clone(),
             cfg.blocks.clone());
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
                for (i, var) in liveness_analyzer.interference_graph.variables.iter().enumerate() {
                    println!("{i} -> {:?}", var);
                } 
            }
            self.interference_graphs.push(liveness_analyzer.interference_graph); 
        }
        
    }

    fn generate_assembly(&mut self) {
        println!("\n---------------------- CODEGEN ---------------------");
        println!("Starting Register Allocation:");
        
        println!("- Allocator created");

        let mut frames = Vec::new();
        for (_, frame) in self.frames.iter() {
            frames.push(frame);
        }

        let mut output = String::new();
        for ig in self.interference_graphs.iter() {
            let mut allocator = new_allocator(ig.clone()); 
            allocator.coloring();
            
            let curr_frame = frames.pop().unwrap();
            let (start, end) = curr_frame.range;

            let mut asm_gen = new_asm_generator(self.instructions[start..=end].to_vec(),
                curr_frame.clone(), 
                allocator.ifr_graph.variables, 
                allocator.spill);
            asm_gen.generate_assembly();
            output.push_str(asm_gen.print_asm().as_str());
        }

        

        if MAKE_ASM_FILE {
            fs::write(ASM_PATH, output).expect("write file failed");
            println!("- TAC file created at '{ASM_PATH}'");
        }

        println!("end");
    }

    fn linker_run(&mut self) {
        println!("\n---------------------- PROGRAM ---------------------");
        println!("- Running GCC: '{}' -> '{}'", ASM_PATH, ASM_CODE_OBJECT);
        let output_gcc = Command::new("gcc")
        .arg("-c").arg(ASM_PATH)
        .arg("-o").arg(ASM_CODE_OBJECT)
        .output().expect("gcc error");

        if !output_gcc.status.success() {
            eprintln!("Linking Error (GCC/Linker):");
            eprintln!("{}", String::from_utf8_lossy(&output_gcc.stderr));
            println!("{}", io::Error::new(io::ErrorKind::Other, "gcc object failed"));
        }
        
        println!("- Running LINKER: '{}' + '{}' -> '{}'", HELPER_FILE, ASM_CODE_OBJECT, EXECUTABLE);
        let output_link = Command::new("gcc")
        .arg(HELPER_FILE).arg(ASM_CODE_OBJECT)
        .arg("-o")
        .arg(EXECUTABLE)
        .output().expect("linker error");

        if !output_link.status.success() {
            eprintln!("Linking Error (GCC/Linker):");
            eprintln!("{}", String::from_utf8_lossy(&output_link.stderr));
            println!("{}", io::Error::new(io::ErrorKind::Other, "exe linking failed"));
        } 

        let now = Instant::now();
        println!("- Running EXE: '{}'", EXECUTABLE);
        let output_run = Command::new(format!("./{}", EXECUTABLE))
        .output().expect("run error");
        io::stdout().write_all(&output_run.stdout).expect("write stdout error");
        io::stdout().write_all(&output_run.stderr).expect("write stderr error");
        if CALCULATE_TIME {
            let end_time = now.elapsed();
            println!("\nrun duration: {} ms", end_time.as_millis());
        }

    }

}