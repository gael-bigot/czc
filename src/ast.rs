use crate::lexer::Token;
use std::fmt::{self, Debug};


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
    Pow,
    Neg,
    Deref,
    AddressOf,
    Cast,
    New,
    Eq,
    Neq,
    And,
    FunctionCall,
}


#[derive(Clone)]
pub struct Expr {
    pub token: Option<Token>,
    pub expr_type: ExprType,
    pub children: Vec<Expr>,
}


pub struct Ident {
    pub token: Token,
}

pub enum ExprAssignment {
    Expr(Expr),
    Assign(Ident, Expr),
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

    pub fn new_binary(expr_type: ExprType, left: Expr, right: Expr) -> Self {
        Self {
            token: None,
            expr_type,
            children: vec![left, right],
        }
    }
    
}



impl Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl Expr {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        // Print indentation
        write!(f, "{:indent$}", "", indent = indent * 2)?;
        
        // Print the expression type
        write!(f, "{:?}", self.expr_type)?;
        
        // Print token if present
        if let Some(token) = &self.token {
            write!(f, " '{}'", token.lexeme)?;
        }
        
        // Print children with increased indentation
        if !self.children.is_empty() {
            writeln!(f)?;
            for child in &self.children {
                child.fmt_with_indent(f, indent + 1)?;
                writeln!(f)?;
            }
        }
        
        Ok(())
    }
}