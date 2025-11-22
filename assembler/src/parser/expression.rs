use crate::ast::Expression;

use super::*;

pub fn expression_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, Expression, Extra<'src>> {
    recursive(|expression| {
        let atom = choice((
            keywords::register_parser().map(Expression::Register),
            string_parser().map(Expression::String),
            number_parser().map(Expression::Number),
            text::ident()
                .map(|s: &str| Intern::new(s.to_string()))
                .map(Expression::Identifier),
            expression
                .padded()
                .delimited_by(just('('), just(')'))
                .labelled("sub-expression"),
        ))
        .map_with(|val, e| Spanned::new(val, e.span()))
        .boxed();

        let unary = choice((
            just('&').to(Expression::Ref as fn(_) -> _),
            just('*').to(Expression::Deref as fn(_) -> _),
            // purposely disallow negatives, as everything should be unsigned
            // (negatives are still used in register offsets, as the cpu treats that as signed)
            // just('-').to(Expression::Neg as fn(_) -> _),
            just('~').to(Expression::Not as fn(_) -> _),
        ))
        .labelled("unary operator")
        .map_with(|val, e| Spanned::new(val, e.span()))
        .padded()
        .repeated()
        .foldr(atom, |op, rhs| {
            let span = op.span.union(&rhs.span); // can't inline because value is moved before span can be created
            Spanned::new(op(Box::new(rhs)), span)
        })
        .boxed();

        let product = unary
            .clone()
            .foldl(
                choice((
                    just('*').to(Expression::Mul as fn(_, _) -> _),
                    just('/').to(Expression::Div as fn(_, _) -> _),
                    just('%').to(Expression::Rem as fn(_, _) -> _),
                ))
                .labelled("binary operator")
                .padded()
                .then(unary.clone())
                .repeated(),
                |lhs, (op, rhs)| {
                    let span = lhs.span.union(&rhs.span);

                    Spanned::new(op(Box::new(lhs), Box::new(rhs)), span)
                },
            )
            .boxed();

        let sum = product
            .clone()
            .foldl(
                choice((
                    just('+').to(Expression::Add as fn(_, _) -> _),
                    just('-').to(Expression::Sub as fn(_, _) -> _),
                ))
                .labelled("binary operator")
                .padded()
                .then(product.clone())
                .repeated(),
                |lhs, (op, rhs)| {
                    let span = lhs.span.union(&rhs.span);

                    Spanned::new(op(Box::new(lhs), Box::new(rhs)), span)
                },
            )
            .boxed();

        let shift = sum
            .clone()
            .foldl(
                choice((
                    just("<<").to(Expression::Shl as fn(_, _) -> _),
                    just(">>>").to(Expression::Ashr as fn(_, _) -> _),
                    just(">>").to(Expression::Shr as fn(_, _) -> _),
                ))
                .labelled("binary operator")
                .padded()
                .then(sum.clone())
                .repeated(),
                |lhs, (op, rhs)| {
                    let span = lhs.span.union(&rhs.span);

                    Spanned::new(op(Box::new(lhs), Box::new(rhs)), span)
                },
            )
            .boxed();

        let and = shift
            .clone()
            .foldl(
                just('&')
                    .labelled("binary operator")
                    .padded()
                    .then(shift.clone())
                    .repeated(),
                |lhs, (_op, rhs)| {
                    let span = lhs.span.union(&rhs.span);

                    Spanned::new(Expression::And(Box::new(lhs), Box::new(rhs)), span)
                },
            )
            .boxed();

        let xor = and
            .clone()
            .foldl(
                just('^')
                    .labelled("binary operator")
                    .padded()
                    .then(and.clone())
                    .repeated(),
                |lhs, (_op, rhs)| {
                    let span = lhs.span.union(&rhs.span);

                    Spanned::new(Expression::Xor(Box::new(lhs), Box::new(rhs)), span)
                },
            )
            .boxed();

        let or = xor
            .clone()
            .foldl(
                just('|')
                    .labelled("binary operator")
                    .padded()
                    .then(xor.clone())
                    .repeated(),
                |lhs, (_op, rhs)| {
                    let span = lhs.span.union(&rhs.span);

                    Spanned::new(Expression::Or(Box::new(lhs), Box::new(rhs)), span)
                },
            )
            .boxed();

        or.map(|expression| expression.val).labelled("expression")
    })
}

pub fn number_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, u32, Extra<'src>> {
    let bin_num =
        just("0b")
            .ignore_then(text::digits(2).collect::<String>().try_map(|s, span| {
                u32::from_str_radix(&s, 2).map_err(|e| ParseError::custom(span, e))
            }))
            .labelled("binary number");

    let oct_num =
        just("0o")
            .ignore_then(text::digits(8).collect::<String>().try_map(|s, span| {
                u32::from_str_radix(&s, 8).map_err(|e| ParseError::custom(span, e))
            }))
            .labelled("octodecimal number");

    let hex_num = just("0x")
        .ignore_then(text::digits(16).collect::<String>().try_map(|s, span| {
            u32::from_str_radix(&s, 16).map_err(|e| ParseError::custom(span, e))
        }))
        .labelled("hexadecimal number");

    #[allow(clippy::from_str_radix_10)]
    let dec_num = text::digits(10)
        .collect::<String>()
        .try_map(|s, span| u32::from_str_radix(&s, 10).map_err(|e| ParseError::custom(span, e)))
        .labelled("decimal number");

    // no need to escape ' or \ since ' and \ can be represented by ''' and '\'
    // we're able to do that because empty chars ('') are not supported
    let escape_char = just('\\')
        .ignore_then(choice((
            just('0').to('\0'),
            just('t').to('\t'),
            just('n').to('\n'),
            just('r').to('\r'),
        )))
        .labelled("escape character");

    let char_num = escape_char
        .or(any())
        .delimited_by(just('\''), just('\''))
        .map(|c| c as u32)
        .labelled("character");

    choice((bin_num, oct_num, hex_num, dec_num, char_num)).labelled("number")
}

pub fn string_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, String, Extra<'src>> {
    let escape_char = just('\\')
        .ignore_then(choice((
            just('\\').to('\\'),
            just('\"').to('"'),
            just('0').to('\0'),
            just('t').to('\t'),
            just('n').to('\n'),
            just('r').to('\r'),
        )))
        .labelled("escape character");

    any()
        .filter(|c| *c != '\\' && *c != '"')
        .or(escape_char)
        .labelled("character")
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .labelled("string")
}
