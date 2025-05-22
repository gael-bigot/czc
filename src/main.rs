use std::env;
mod ast;
mod lexer;
mod parser;


fn run(input: &str, file_name: &str) {
    let (tokens, errors) = lexer::lex(input, file_name);
    if errors > 0 {
        panic!("Lexing failed with {} errors", errors);
    }
    for token in tokens {
        println!("{:?}: {}", token.kind, token.lexeme);
    }
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