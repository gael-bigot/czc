use std::env;
mod lexer;


fn run(input: &str) {
    let tokens = lexer::lex(input);
    for token in tokens {
        println!("{:?}: {}", token.kind, token.lexeme);
    }
}

fn from_file(path: &str) {
    let contents = std::fs::read_to_string(path).expect("Could not read file.");
    run(&contents);
}


fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        from_file(&args[1]);
    } else {
        panic!("No file provided");
    }
}