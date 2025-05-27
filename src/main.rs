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
    //for instruction in assembler.casm.clone() {
    //    println!("{:?}", instruction);
    //}
    
    assembler.build_instructions();
    // for instruction in assembler.instructions.clone() {
    //     println!("{:?}", instruction);
    // }
    let json = assembler.to_json();
    println!("{}", json);
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