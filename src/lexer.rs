use ariadne::{Color, Label, Report, ReportKind, Source};
use logos::Logos;


#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[\t\n ]+")]
#[logos(skip r"//.*\n")]
pub enum Token {

    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Int(u64),

    #[regex(r"0x[0-9a-fA-F]+", |lex| u64::from_str_radix(&lex.slice()[2..], 16).ok())]
    HexInt(u64),
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),
    #[regex(r#"".""#, |lex| Some(lex.slice().to_string()))]
    String(String),
    #[regex(r"'.'", |lex| Some(lex.slice().to_string()))]
    ShortString(String),

    #[token("++")]
    PlusPlus,
    #[token("==")]
    DoubleEq,
    #[token("**")]
    DoubleStar,
    #[token("!=")]
    Neq,
    #[token("->")]
    Arrow,
    #[token("@")]
    At,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("felt")]
    Felt,
    #[token("codeoffset")]
    CodeOffset,

    #[token("*")]
    Star,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    
    #[token("=")]
    Equal,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,

    #[token("&")]
    Ampersand,
    #[token("new")]
    New,

    #[token(".")]
    Dot,

    //Atom
    #[token("nondet")]
    NonDet,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("cast")]
    Cast,

    //Reg
    #[token("ap")]
    Ap,
    #[token("fp")]
    Fp,

    #[token("and")]
    And,
    

    #[token("local")]
    Local,

    #[token("ret")]
    Ret,

    //Instructions

    #[token("call")]
    Call,
    #[token("rel")]
    Rel,
    #[token("abs")]
    Abs,
    #[token("jmp")]
    Jmp,
    #[token("if")]
    If,
    #[token("+=")]
    PlusEq,
    #[token("dw")]
    Dw,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    // Import statement
    #[token("import")]
    Import,
    #[token("from")]
    From,
    #[token("as")]
    As,

    // Function/Namespace/Struct definition.
    #[token("func")]
    Func,
    #[token("with")]
    With,
    #[token("struct")]
    Struct,
    #[token("namespace")]
    Namespace,
    #[token("with_attr")]
    WithAttr,

    // Cairo file
    // #[token("\n")]
    // Newline,


    // Code elements
    #[token(";")]
    Semicolon,
    #[token("const")]
    Const,
    #[token("let")]
    Let,
    #[token("tempvar")]
    TempVar,
    #[token("assert")]
    Assert,
    #[token("static_assert")]
    StaticAssert,
    #[token("return")]
    Return,
    #[token("using")]
    Using,
    #[token("alloc_locals")]
    AllocLocals,

}




pub fn lex(input: &str, file_name: &str) -> (Vec<Token>, u32) {
    let mut error_counter = 0;
    let mut lex = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lex.next() {
        let lexeme = lex.slice().to_string();
        if let Ok(token) = token {
            tokens.push(token);
        } else {
            let error_span = (file_name, lex.span().start..lex.span().end);
            let _ = Report::build(ReportKind::Error, error_span.clone())
                .with_message("Lexer error")
                .with_label(Label::new(error_span)
                    .with_message(format!("Unknown token '{}'", lexeme))
                    .with_color(Color::Red))
                .finish()
                .print((file_name, Source::from(input)));
            error_counter += 1;
        }
    }
    (tokens, error_counter)
}