use logos::Logos;


pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String) -> Self {
        Self { kind, lexeme }
    }
}


// Token enum for logos lexer
// The actual base token object for the compiler is Token
#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \n\t]+")]
#[logos(skip r"//.*\n")]
pub enum TokenType {
    // Keywords
    #[token("func")]
    Func,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    #[token("local")]
    Local,
    #[token("const")]
    Const,
    #[token("struct")]
    Struct,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("with_attr")]
    WithAttr,
    #[token("alloc_locals")]
    AllocLocals,
    #[token("from")]
    From,
    #[token("import")]
    Import,

    // Builtins
    #[token("%builtins")]
    Builtins,
    #[token("output")]
    Output,
    #[token("pedersen")]
    Pedersen,
    #[token("range_check")]
    RangeCheck,
    #[token("ecdsa")]
    Ecdsa,
    #[token("bitwise")]
    Bitwise,

    // Punctuation
    #[token(";")]
    Semicolon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("&")]
    Ampersand,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token("!=")]
    NotEqual,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token("++")]
    PlusPlus,
    #[token("%")]
    Percent,

    // Literals
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Special tokens
    #[token("%{")]
    HintStart,
    #[token("%}")]
    HintEnd,
    #[token("ap")]
    Ap,
    #[token("fp")]
    Fp,
    #[token("felt")]
    Felt,
    #[token("cast")]
    Cast,
    #[token("assert")]
    Assert,
    #[token("get_fp_and_pc")]
    GetFpAndPc,
    #[token("alloc")]
    Alloc,
    #[token("jmp")]
    Jmp,
    #[token("abs")]
    Abs,
    #[token("rel")]
    Rel,
    #[token("case")]
    Case,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Nil,
}



pub fn lex(input: &str) -> Vec<Token> {
    let mut lex = TokenType::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lex.next() {
        if let Ok(token_type) = token {
            tokens.push(Token::new(token_type, lex.slice().to_string()));
        }
    }
    tokens
}