use std::env;
use chumsky::prelude::*;
mod ast;
mod lexer;
mod parser;


fn run(input: &str, file_name: &str) {
    let (tokens, errors) = lexer::lex(input, file_name);
    if errors > 0 {
        panic!("Lexing failed with {} errors", errors);
    }
    let expr = parser::parser().parse(&tokens);
    println!("{:?}", expr);
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