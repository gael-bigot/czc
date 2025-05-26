use std::env;
mod ast;
mod lexer;
mod parser;
mod error;

extern crate ebnf;


fn run(input: &str, file_name: &str) {
    
    let (tokens, errors) = lexer::lex(input, file_name);
    if errors > 0 {
        panic!("Lexing failed with {} errors", errors);
    }
    println!("{:?}", tokens);
    let mut parser = parser::Parser::new(tokens, file_name.to_string(), input.to_string());
    let code_elements = parser.parse();
    code_elements.iter().for_each(|code_element| println!("{:?}", code_element));
    
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