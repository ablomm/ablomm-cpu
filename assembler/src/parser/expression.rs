use super::*;

pub fn expression_parser() -> impl Parser<char, Expression, Error = Error> {
    let bin_num = just("0b")
        .ignore_then(text::digits(2).map(|s: String| i64::from_str_radix(&s, 2).unwrap()));
    let oct_num = just("0o")
        .ignore_then(text::digits(8).map(|s: String| i64::from_str_radix(&s, 8).unwrap()));
    let hex_num = just("0x")
        .ignore_then(text::digits(16).map(|s: String| i64::from_str_radix(&s, 16).unwrap()));
    let dec_num = text::digits(10).map(|s: String| i64::from_str_radix(&s, 10).unwrap());

    // no need to escape ' or \ since ' and \ can be represented by ''' and '\'
    // we're able to do that because empty chars ('') are not supported
    let escape_char = just('\\').ignore_then(choice((
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
        just('0').to('\0'),
    )));

    let char_num = escape_char
        .or(any())
        .delimited_by(just('\''), just('\''))
        .map(|c| c as i64);

    let number = choice((bin_num, oct_num, hex_num, dec_num, char_num));

    let expr = recursive(|expression| {
        let atom = just('+')
            .padded()
            .or_not()
            .ignore_then(choice((
                number.map(Expression::Number),
                text::ident().map(Expression::Ident),
                expression.delimited_by(just('('), just(')')),
            )))
            .map_with_span(Spanned::new);

        let unary = just('-')
            .map_with_span(Spanned::new)
            .padded()
            .repeated()
            .then(atom)
            .foldr(|op, rhs| {
                let span = op.span.union(&rhs.span); // can't inline because value is moved before span
                                                     // can be created
                Spanned::new(Expression::Neg(Box::new(rhs)), span)
            });

        let product = unary
            .clone()
            .then(
                choice((
                    just('*').padded().to(Expression::Mul as fn(_, _) -> _),
                    just('/').padded().to(Expression::Div as fn(_, _) -> _),
                ))
                .map_with_span(Spanned::new)
                .then(unary.clone())
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(&rhs.span);
                return Spanned::new(op(Box::new(lhs), Box::new(rhs)), span);
            });

        let sum = product
            .clone()
            .then(
                choice((
                    just('+').padded().to(Expression::Add as fn(_, _) -> _),
                    just('-').padded().to(Expression::Sub as fn(_, _) -> _),
                ))
                .map_with_span(Spanned::new)
                .then(product.clone())
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(&rhs.span);
                return Spanned::new(op(Box::new(lhs), Box::new(rhs)), span);
            });

        return sum.map(|sum| sum.val);
    });

    return expr;
}
