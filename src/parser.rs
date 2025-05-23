use chumsky::prelude::*;
use crate::ast::*;
use crate::lexer::Token;


pub fn expr_parser<'src>() -> impl Parser<'src, &'src [Token], Expr<'src>> {
    let expr = recursive(|expr| {

        let regexpr = choice((
            just(Token::Ap).to(Expr::Register(Register::Ap)),
            just(Token::Fp).to(Expr::Register(Register::Fp)),
        ));

        let atom = choice((
            regexpr,
            // Literals
            select! {
                Token::Int(n) => Expr::IntegerLiteral(n),
                Token::HexInt(n) => Expr::IntegerLiteral(n),
                Token::ShortString(s) => Expr::IntegerLiteral(s.parse().unwrap_or(0)),
                Token::Identifier(s) => Expr::Identifier(s.to_owned().leak()),
            },
            // Parenthesized expressions
            expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)),
            // Dereference
            just(Token::LBracket).then(expr.clone()).then(just(Token::RBracket)).map(|((_, ptr), _)| Expr::Deref(Box::new(ptr))),
            // Cast
            just(Token::Cast)
                .then(just(Token::LParen))
                .then(expr.clone())
                .then(just(Token::Comma))
                .then(expr)
                .then(just(Token::RParen)).map(|((((_, lhs), _), rhs), _)| Expr::Cast(Box::new(lhs), Box::new(rhs))),
        ));


        let unary = choice((
            just(Token::Minus),
            just(Token::Ampersand),
            just(Token::New),
        )).repeated().foldr(atom, |op, rhs| match op {
            Token::Minus => Expr::Minus(Box::new(rhs)),
            Token::Ampersand => Expr::AddressOf(Box::new(rhs)),
            Token::New => Expr::New(Box::new(rhs)),
            _ => unreachable!(),
        });


        let product = unary.clone().foldl(
            choice((
                just(Token::Star).then(unary.clone()),
                just(Token::Slash).then(unary.clone()),
            )).repeated(),
            |lhs, (op, rhs)| match op {
                Token::Star => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                Token::Slash => Expr::Div(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            }
        );

        let sum = product.clone().foldl(
            choice((
                just(Token::Plus).then(product.clone()),
                just(Token::Minus).then(product.clone()),
            )).repeated(),
            |lhs, (op, rhs)| match op { 
                Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            }
        );

        let bool_atom = sum.clone().foldl(
            choice((
                just(Token::DoubleEq).then(sum.clone()),
                just(Token::Neq).then(sum.clone()),
            )).repeated(),
            |lhs, (op, rhs)| match op {
                Token::DoubleEq => Expr::Eq(Box::new(lhs), Box::new(rhs)),
                Token::Neq => Expr::Neq(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            }
        );

        let bool_and = bool_atom.clone().foldl(
            choice((
                just(Token::And).then(bool_atom.clone()),
            )).repeated(),
            |lhs, (op, rhs)| match op {
                Token::And => Expr::And(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            }
        );

        bool_and
    });

    expr
}

fn assignement_expression_parser<'src>() -> impl Parser<'src, &'src [Token], ExprAssignment<'src>> {
    let assignement = select! {
        Token::Identifier(s) => s
    }.then(just(Token::Equal))
     .then(expr_parser())
     .map(|((name, _), rhs)| ExprAssignment::Assign(name.to_owned().leak(), Box::new(rhs)));
    assignement.or(expr_parser().map(|expr| ExprAssignment::Expr(Box::new(expr))))
}

fn typed_identifier_parser<'src>() -> impl Parser<'src, &'src [Token], TypedIdentifier<'src>> {
    // Parse optional modifier
    let modifier = just(Token::Local)
        .to(Modifier::Local)
        .or_not();

    // Parse identifier name
    let name = select! {
        Token::Identifier(s) => s.to_string().leak()
    };

    // Parse optional type annotation
    let type_ = just(Token::Colon)
        .then(type_parser())
        .map(|(_, t)| t)
        .or_not();

    // Combine all parts
    modifier
        .then(name)
        .then(type_)
        .map(|((modifier, name), type_)| TypedIdentifier {
            modifier,
            name,
            type_,
        })
}

/*
fn named_type_parser<'src>() -> impl Parser<'src, &'src [Token], NamedType<'src>> {
    
}
*/

// Type system parsers
fn type_parser<'src>() -> impl Parser<'src, &'src [Token], Type<'src>> {
    let named_type_parser = select! {
        Token::Identifier(s) => s.to_string().leak()
    }
    .then(just(Token::Colon).then(type_parser()).map(|(_, t)| t).or_not())
    .map(|(name, type_)| NamedType { name, type_ });

    recursive(|type_parser| {
        // Parse non-identifier types
        let non_identifier_type = choice((
            // felt
            just(Token::Felt).to(Type::Felt),
            // codeoffset
            just(Token::CodeOffset).to(Type::CodeOffset),
            // pointer (type *)
            type_parser.clone().then(just(Token::Star)).map(|(t, _)| Type::Pointer(Box::new(t))),

            type_parser.clone().then(just(Token::DoubleStar)).map(|(t, _)| Type::Pointer2(Box::new(t))),
 
            named_type_parser
                .separated_by(just(Token::Comma))
                .collect()
                .delimited_by(just(Token::LParen), just(Token::RParen))
                .map(|types| Type::Tuple(types)),
        ));
        // Combine both types
    non_identifier_type
    })
    
}

/*
// Instruction parsers
fn instruction_parser<'src>() -> impl Parser<'src, &'src [Token], Instruction<'src>> {
    todo!("Implement instruction parser")
}

fn call_instruction_parser<'src>() -> impl Parser<'src, &'src [Token], CallInstruction<'src>> {
    todo!("Implement call instruction parser")
}

// Code element parsers
fn code_element_parser<'src>() -> impl Parser<'src, &'src [Token], CodeElement<'src>> {
    todo!("Implement code element parser")
}

fn function_parser<'src>() -> impl Parser<'src, &'src [Token], Function<'src>> {
    todo!("Implement function parser")
}

fn struct_parser<'src>() -> impl Parser<'src, &'src [Token], Struct<'src>> {
    todo!("Implement struct parser")
}

fn namespace_parser<'src>() -> impl Parser<'src, &'src [Token], Namespace<'src>> {
    todo!("Implement namespace parser")
}

fn import_parser<'src>() -> impl Parser<'src, &'src [Token], Import<'src>> {
    todo!("Implement import parser")
}


*/