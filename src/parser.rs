use crate::ast::*;
use crate::lexer::Token;
use ariadne::{Color, Label, Report, ReportKind, Source};




pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    source: String,
    file_name: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file_name: String, source: String) -> Self {
        Self { tokens, current: 0, source, file_name }
    }

    fn match_token(&mut self, token_type: crate::lexer::TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&mut self, token_type: crate::lexer::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == crate::lexer::TokenType::EOF
    }

    fn advance(&mut self) -> crate::lexer::Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }

    fn peek(&mut self) -> crate::lexer::Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> crate::lexer::Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: crate::lexer::TokenType, message: &str) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            let previous_token_span = self.previous().span;
            let error_span = (self.file_name.clone(), previous_token_span.0..previous_token_span.1);
            let _ = Report::build(ReportKind::Error, error_span.clone())
                .with_message("Syntax error")
                .with_label(Label::new(error_span)
                    .with_message(message)
                    .with_color(Color::Red))
                .finish()
                .print((self.file_name.clone(), Source::from(self.source.clone())));
            Token {
                token_type: crate::lexer::TokenType::Error,
                lexeme: "".to_string(),
                span: (previous_token_span.0, previous_token_span.0),
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        while !self.is_at_end() {
            exprs.push(self.expression());
        }
        exprs
    }

    fn expression(&mut self) -> Expr {
        self.sum()
    }

    /*
    fn arglist_list(&mut self) -> Vec<ExprAssignment> {
        let mut args = Vec::new();
        while !self.check(crate::lexer::TokenType::RParen) {
            args.push(self.expression());
        }
        args
    }
    */

    fn sum(&mut self) -> Expr {
        let mut expr = self.product();
        while self.check(crate::lexer::TokenType::Plus) || self.check(crate::lexer::TokenType::Minus) {
            let operator = self.advance();
            let right = self.product();
            match operator.token_type {
                crate::lexer::TokenType::Plus => {
                    expr = Expr::new_binary(ExprType::Add, expr, right);
                }
                crate::lexer::TokenType::Minus => {
                    expr = Expr::new_binary(ExprType::Sub, expr, right);
                }
                _ => unreachable!(),
            }
        }
        expr
    }

    fn product(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.check(crate::lexer::TokenType::Star) || self.check(crate::lexer::TokenType::Slash) {
            let operator = self.advance();
            let right = self.expression();
            match operator.token_type {
                crate::lexer::TokenType::Star => {
                    expr = Expr::new_binary(ExprType::Mul, expr, right);
                }
                crate::lexer::TokenType::Slash => {
                    expr = Expr::new_binary(ExprType::Div, expr, right);
                }
                _ => unreachable!(),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        let next = self.peek();
        match next.token_type {
            crate::lexer::TokenType::Ampersand => {
                self.advance();
                let right = self.unary();
                Expr::new_unary(ExprType::AddressOf, right)
            }
            crate::lexer::TokenType::Minus => {
                self.advance();
                let right = self.unary();
                Expr::new_unary(ExprType::Neg, right)
            }
            crate::lexer::TokenType::New => {
                self.advance();
                let right = self.unary();
                Expr::new_unary(ExprType::New, right)
            }
            _ => self.pow(),
        }
    }

    fn pow(&mut self) -> Expr {
        let mut expr = self.atom();
        while self.check(crate::lexer::TokenType::DoubleStar) {
            self.advance();
            let right = self.expression();
            expr = Expr::new_binary(ExprType::Pow, expr, right);
        }
        expr
    }

    fn atom(&mut self) -> Expr {
        let token = self.advance();
        match token.token_type {
            crate::lexer::TokenType::Int => {
                Expr::new_terminal(ExprType::IntegerLiteral, token)
            }
            crate::lexer::TokenType::Identifier => {
                Expr::new_terminal(ExprType::Identifier, token)
            }
            crate::lexer::TokenType::HexInt => {
                Expr::new_terminal(ExprType::IntegerLiteral, token)
            }
            crate::lexer::TokenType::ShortString => {
                Expr::new_terminal(ExprType::IntegerLiteral, token)
            }
            crate::lexer::TokenType::NonDet => {
                let hint = self.consume(crate::lexer::TokenType::NonDet, "Expected hint after nondet");
                Expr::new_terminal(ExprType::Hint, hint)
            }
            crate::lexer::TokenType::Ap | crate::lexer::TokenType::Fp => {
                Expr::new_terminal(ExprType::Register, token)
            }
            // TODO function call
            crate::lexer::TokenType::LBracket => {
                let expr = self.expression();
                self.consume(crate::lexer::TokenType::RBracket, "Expected ']' after dereferencing");
                Expr::new_unary(ExprType::Deref, expr)
            }
            crate::lexer::TokenType::LParen => {
                let expr = self.expression();
                self.consume(crate::lexer::TokenType::RParen, "Expected ')' after expression");
                expr
            }
            // TODO subscript
            // TODO dot
            // TODO cast
            // TODO arglist
            _ => todo!(),
        }
    }
}