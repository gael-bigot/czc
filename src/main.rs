use std::env;
mod ast;
mod lexer;
mod parser;
mod error;
mod casm;
mod lower_to_casm;
mod assembler;

extern crate ebnf;


fn run(input: &str, file_name: &str) {
    
    
    let (tokens, errors) = lexer::lex(input, file_name);
    if errors > 0 {
        panic!("Lexing failed with {} errors", errors);
    }
    //println!("{:?}", tokens);
    let mut parser = parser::Parser::new(tokens, file_name.to_string(), input.to_string());
    let code_elements = parser.parse();
    //code_elements.iter().for_each(|code_element| println!("{:?}", code_element));
    let mut compiler = lower_to_casm::Compiler::new(code_elements);
    let casm = compiler.compile();
    let mut assembler = assembler::Compiler::new();
    assembler.casm = casm;
    assembler.resolve_calls();
    for instruction in assembler.casm.clone() {
        println!("{:?}", instruction);
    }
    
    assembler.build_instructions();
    for instr in assembler.instructions.clone() {
        println!("{:?}", instr);
    }
    for instruction in assembler.instructions.clone() {
        let (bytes, imm) = instruction.to_bytes();
        println!("{:#x}", bytes);
        if let Some(imm) = imm {
            println!("{:#x}", imm);
        }
    }
    let json = assembler.to_json();
    println!("{}", json);
    
    /*
    let instr = assembler::Instruction{
        offdst: 2,
        offop0: -1,
        offop1: -2,
        imm: None,
        dst: 1,
        op0: 1,
        op1: 2,
        res: 2,
        pc_update: 0,
        ap_update: 2,
        opcode: 4,
    };
    let (bytes, imm) = instr.to_bytes();
    println!("{:#x}", bytes);
    if let Some(imm) = imm {
        println!("{:#x}", imm);
    }
    */
}

fn from_file(path: &str) {
    let contents = std::fs::read_to_string(path).expect("Could not read file.");
    run(&contents, path);
}

fn main(){
    
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        from_file(&args[1]);
    } else {
        panic!("No file provided");
    }
    
}