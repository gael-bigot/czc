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
    Subscript,
    Tuple,
    TupleOrParen,
    ErrorExpr,
}


#[derive(Clone)]
pub struct Expr {
    pub token: Option<Token>,
    pub ident: Option<Identifier>,
    pub expr_type: ExprType,
    pub left: Option<Box<Expr>>,
    pub right: Option<Box<Expr>>,
    pub args: Vec<ExprAssignment>,
}



#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
}

#[derive(Clone)]
pub enum ExprAssignment {
    Expr(Expr),
    Assign(Identifier, Expr),
}








impl Debug for ExprAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl ExprAssignment {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        match self {
            ExprAssignment::Expr(expr) => {
                expr.fmt_with_indent(f, indent)
            }
            ExprAssignment::Assign(ident, expr) => {
                write!(f, "{:indent$}", "", indent = indent * 2)?;
                write!(f, "Assign '{}' = ", ident.token.lexeme)?;
                writeln!(f)?;
                write!(f, "{:indent$}", "", indent = indent * 2)?;
                expr.fmt_with_indent(f, indent)
            }
        }
    }
}

impl Expr {
    pub fn new_error() -> Self {
        Self {
            token: None,
            ident: None,
            expr_type: ExprType::ErrorExpr,
            left: None,
            right: None,
            args: vec![],
        }
    }
    pub fn new_identifier(ident: Identifier) -> Self {
        Self {
            token: None,
            ident: Some(ident),
            expr_type: ExprType::Identifier,
            left: None,
            right: None,
            args: vec![],
        }
    }

    pub fn new_terminal(expr_type: ExprType, token: Token) -> Self {
        Self {
            token: Some(token),
            ident: None,
            expr_type,
            left: None,
            right: None,
            args: vec![],
        }
    }

    pub fn new_unary(expr_type: ExprType, child: Expr) -> Self {
        Self {
            token: None,
            ident: None,
            expr_type,
            left: Some(Box::new(child)),
            right: None,
            args: vec![],
        }
    }

    pub fn new_binary(expr_type: ExprType, left: Expr, right: Expr) -> Self {
        Self {
            token: None,
            ident: None,
            expr_type,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            args: vec![],
        }
    }

    pub fn new_function_call(name: String, args: Vec<ExprAssignment>) -> Self {
        Self {
            token: None,
            ident: None,
            expr_type: ExprType::FunctionCall,
            left: None,
            right: None,
            args,
        }
    }

    pub fn new_tuple_or_paren(args: Vec<ExprAssignment>) -> Self {
        Self {
            token: None,
            ident: None,
            expr_type: ExprType::TupleOrParen,
            left: None,
            right: None,
            args,
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

        // Print identifier if present
        if let Some(ident) = &self.ident {
            write!(f, " '{}'", ident.token.lexeme)?;
        }

        // Print children with increased indentation
        let has_children = self.left.is_some() || self.right.is_some() || !self.args.is_empty();
        if has_children {
            writeln!(f)?;
            
            // Print left child if present
            if let Some(left) = &self.left {
                left.fmt_with_indent(f, indent + 1)?;
                writeln!(f)?;
            }
            
            // Print right child if present
            if let Some(right) = &self.right {
                right.fmt_with_indent(f, indent + 1)?;
                writeln!(f)?;
            }
            
            // Print args if present
            for arg in &self.args {
                arg.fmt_with_indent(f, indent + 1)?;
                writeln!(f)?;
            }
        }
        
        Ok(())
    }
}