use std::env;
mod assembler;
mod ast;
mod casm;
mod error;
mod lexer;
mod lower_to_casm;
mod minivm;
mod parser;

extern crate ebnf;

fn run(input: &str, file_name: &str) {
    let (tokens, errors) = lexer::lex(input, file_name);
    if errors > 0 {
        panic!("Lexing failed with {} errors", errors);
    }

    let mut parser = parser::Parser::new(tokens, file_name.to_string(), input.to_string());
    let code_elements = parser.parse();

    let mut compiler = lower_to_casm::Compiler::new(code_elements);
    let casm = compiler.compile();
    for (i, instruction) in casm.clone().iter().enumerate() {
        println!("{} {:?}", i, instruction);
    }
    let mut assembler = assembler::Assembler::new();
    assembler.casm = casm;
    assembler.resolve_jumps();
    for (i, instruction) in assembler.casm.clone().iter().enumerate() {
        println!("{} {:?}", i, instruction);
    }

    assembler.build_instructions();

    let mut line = 0;
    for instruction in assembler.instructions.clone() {
        let (bytes, imm) = instruction.to_bytes();
        println!("{} {:#x}", line, bytes);
        if let Some(imm) = imm {
            line += 1;
            println!("{} {:#x}", line, imm);
        }
        line += 1;
    }
    let json = assembler.to_json();
    println!("{}", json);
}

fn from_file(path: &str) {
    let contents = std::fs::read_to_string(path).expect("Could not read file.");
    run(&contents, path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        from_file(&args[1]);
    } else {
        panic!("No file provided");
    }
}
