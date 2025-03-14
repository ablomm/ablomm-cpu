use super::*;

pub fn expression_parser() -> impl Parser<char, Expression, Error = ParseError> {
    let expr = recursive(|expression| {
        let atom = choice((
            keywords::register_parser().map(Expression::Register),
            string_parser().map(Expression::String),
            number_parser().map(Expression::Number),
            text::ident().map(Intern::new).map(Expression::Ident),
            expression.delimited_by(just('('), just(')')),
        ))
        .map_with_span(Spanned::new)
        .boxed();

        let unary = choice((
            just('&').to(Expression::Ref as fn(_) -> _),
            just('*').to(Expression::Deref as fn(_) -> _),
            // purposely disallow negatives, as everything should be unsigned
            // (negatives are still used in register offsets, as the cpu treats that as signed)
            // just('-').to(Expression::Neg as fn(_) -> _),
            just('~').to(Expression::Not as fn(_) -> _),
        ))
        .padded()
        .map_with_span(Spanned::new)
        .repeated()
        .then(atom.padded())
        .foldr(|op, rhs| {
            let span = op.span.union(&rhs.span); // can't inline because value is moved before span
                                                 // can be created
            Spanned::new(op(Box::new(rhs)), span)
        })
        .boxed();

        let product = unary
            .clone()
            .then(
                choice((
                    just('*').to(Expression::Mul as fn(_, _) -> _),
                    just('/').to(Expression::Div as fn(_, _) -> _),
                    just('%').to(Expression::Remainder as fn(_, _) -> _),
                ))
                .padded()
                .map_with_span(Spanned::new)
                .then(unary.clone())
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(&rhs.span);

                Spanned::new(op(Box::new(lhs), Box::new(rhs)), span)
            })
            .boxed();

        let sum = product
            .clone()
            .then(
                choice((
                    just('+').to(Expression::Add as fn(_, _) -> _),
                    just('-').to(Expression::Sub as fn(_, _) -> _),
                ))
                .padded()
                .map_with_span(Spanned::new)
                .then(product.clone())
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(&rhs.span);

                Spanned::new(op(Box::new(lhs), Box::new(rhs)), span)
            })
            .boxed();

        let shift = sum
            .clone()
            .then(
                choice((
                    just("<<").to(Expression::Shl as fn(_, _) -> _),
                    just(">>>").to(Expression::Ashr as fn(_, _) -> _),
                    just(">>").to(Expression::Shr as fn(_, _) -> _),
                ))
                .padded()
                .map_with_span(Spanned::new)
                .then(sum.clone())
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.union(&rhs.span);

                Spanned::new(op(Box::new(lhs), Box::new(rhs)), span)
            })
            .boxed();

        let and = shift
            .clone()
            .then(just('&').padded().then(shift.clone()).repeated())
            .foldl(|lhs, (_op, rhs)| {
                let span = lhs.span.union(&rhs.span);

                Spanned::new(Expression::And(Box::new(lhs), Box::new(rhs)), span)
            })
            .boxed();

        let xor = and
            .clone()
            .then(just('^').padded().then(and.clone()).repeated())
            .foldl(|lhs, (_op, rhs)| {
                let span = lhs.span.union(&rhs.span);

                Spanned::new(Expression::Xor(Box::new(lhs), Box::new(rhs)), span)
            })
            .boxed();

        let or = xor
            .clone()
            .then(just('|').padded().then(xor.clone()).repeated())
            .foldl(|lhs, (_op, rhs)| {
                let span = lhs.span.union(&rhs.span);

                Spanned::new(Expression::Or(Box::new(lhs), Box::new(rhs)), span)
            })
            .boxed();

        or.map(|expression| expression.val)
    });

    expr
}

pub fn number_parser() -> impl Parser<char, u32, Error = ParseError> {
    let bin_num = just("0b").ignore_then(text::digits(2).try_map(|s: String, span| {
        u32::from_str_radix(&s, 2).map_err(|e| ParseError::custom(span, format!("{}", e)))
    }));

    let oct_num = just("0o").ignore_then(text::digits(8).try_map(|s: String, span| {
        u32::from_str_radix(&s, 8).map_err(|e| ParseError::custom(span, format!("{}", e)))
    }));

    let hex_num = just("0x").ignore_then(text::digits(16).try_map(|s: String, span| {
        u32::from_str_radix(&s, 16).map_err(|e| ParseError::custom(span, format!("{}", e)))
    }));

    #[allow(clippy::from_str_radix_10)]
    let dec_num = text::digits(10).try_map(|s: String, span| {
        u32::from_str_radix(&s, 10).map_err(|e| ParseError::custom(span, format!("{}", e)))
    });

    // no need to escape ' or \ since ' and \ can be represented by ''' and '\'
    // we're able to do that because empty chars ('') are not supported
    let escape_char = just('\\').ignore_then(choice((
        just('0').to('\0'),
        just('t').to('\t'),
        just('n').to('\n'),
        just('r').to('\r'),
    )));

    let char_num = escape_char
        .or(any())
        .delimited_by(just('\''), just('\''))
        .map(|c| c as u32);

    choice((bin_num, oct_num, hex_num, dec_num, char_num))
}
