use crate::lexer::Token;


#[derive(Debug, Clone)]
pub enum ExprType {
    IntegerLiteral,
    Hint,
    Identifier,
    Register,
    Add,
    Sub,
    Mul,
    Div,
    Minus,
    Deref,
    AddressOf,
    Cast,
    New,
    Eq,
    Neq,
    And,
    FunctionCall,
}


#[derive(Debug, Clone)]
pub struct Expr {
    pub token: Option<Token>,
    pub expr_type: ExprType,
    pub children: Vec<Expr>,
}



impl Expr {
    pub fn new_terminal(expr_type: ExprType, token: Token) -> Self {
        Self {
            token: Some(token),
            expr_type,
            children: vec![],
        }
    }

    pub fn new_unary(expr_type: ExprType, child: Expr) -> Self {
        Self {
            token: None,
            expr_type,
            children: vec![child],
        }
    }
    
}