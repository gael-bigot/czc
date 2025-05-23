use crate::lexer::Token;
use std::fmt::{self, Debug};



#[derive(Debug, Clone)]
pub enum NamedType{
    Identifier(Identifier, Option<Type>),
    Type(Type),
}

#[derive(Clone)]
pub enum Type{
    Felt,
    CodeOffset,
    Pointer(Box<Type>),
    Pointer2(Box<Type>),
    Tuple(Vec<Type>),
    Struct(Identifier),
    Named(Identifier, Box<Type>),
    Error,
}



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
    pub type_arg: Option<Type>,
    pub args: Vec<ExprAssignment>,
}



#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
}

#[derive(Clone)]
pub enum ExprAssignment {
    Expr(Expr),
    Assign(Identifier, Expr),
}



impl Type {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        write!(f, "{:indent$}", "", indent = indent * 2)?;
        match self {
            Type::Felt => write!(f, "Felt"),
            Type::CodeOffset => write!(f, "CodeOffset"),
            Type::Pointer(inner) => {
                writeln!(f, "Pointer")?;
                inner.fmt_with_indent(f, indent + 1)
            }
            Type::Pointer2(inner) => {
                writeln!(f, "Pointer2")?;
                inner.fmt_with_indent(f, indent + 1)
            }
            Type::Tuple(types) => {
                writeln!(f, "Tuple")?;
                for t in types {
                    t.fmt_with_indent(f, indent + 1)?;
                    writeln!(f)?;
                }
                Ok(())
            }
            Type::Struct(ident) => {
                writeln!(f, "Struct '{}'", ident.token.lexeme)
            }
            Type::Named(ident, inner) => {
                writeln!(f, "Named '{}'", ident.token.lexeme)?;
                inner.fmt_with_indent(f, indent + 1)
            }
            Type::Error => write!(f, "Error"),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
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
            type_arg: None,
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
            type_arg: None,
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
            type_arg: None,
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
            type_arg: None,
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
            type_arg: None,
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
            type_arg: None,
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
            type_arg: None,
            args,
        }
    }

    pub fn new_cast(type_arg: Type, child: Expr) -> Self {
        Self {
            token: None,
            ident: None,
            expr_type: ExprType::Cast,
            left: Some(Box::new(child)),
            right: None,
            type_arg: Some(type_arg),
            args: vec![],
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

            // Print right child if present
            if let Some(type_arg) = &self.type_arg {
                type_arg.fmt_with_indent(f, indent + 1)?;
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