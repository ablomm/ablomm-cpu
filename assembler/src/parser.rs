use crate::ast::*;
use crate::Error;
use chumsky::combinator::MapWithSpan;
use chumsky::prelude::*;
use expression::*;
use keywords::*;
use std::char;
use text::TextParser;

mod expression;
mod keywords;

pub fn parser() -> impl Parser<char, Vec<Spanned<Statement>>, Error = Error> {
    return statement_parser()
        .map_with_span(Spanned::new)
        .repeated()
        .then_ignore(end());
}

fn comment_parser() -> impl Parser<char, String, Error = Error> {
    let line_comment = just("//").ignore_then(take_until(just("\n")).padded());
    let multiline_comment = just("/*").ignore_then(take_until(just("*/")).padded());
    return line_comment
        .or(multiline_comment)
        .map(|(_, comment)| comment.into());
}

fn string_parser() -> impl Parser<char, String, Error = Error> {
    let escape_string = just('\\').ignore_then(choice((
        just('\\').to('\\'),
        just('\"').to('"'),
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
        just('0').to('\0'),
    )));

    return filter(|c| *c != '\\' && *c != '"')
        .or(escape_string)
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'));
}

fn operation_parser() -> impl Parser<char, Operation, Error = Error> {
    let parameter = recursive(|parameter| {
        let register_offset = register_parser()
            .map_with_span(Spanned::new)
            .then(
                choice((
                    just('+').to(Expression::Pos as fn(_) -> _),
                    just('-').to(Expression::Neg as fn(_) -> _),
                ))
                .padded()
                .map_with_span(Spanned::new),
            )
            .then(expression_parser().map_with_span(Spanned::new))
            .map(|((register, op), expression)| {
                let span = op.span.union(&expression.span);
                Parameter::RegisterOffset(
                    register,
                    Spanned::new(op(Box::new(Spanned::new(expression.val, span))), span),
                )
            });

        let indirect = parameter.delimited_by(just('['), just(']'));
        return choice((
            register_offset,
            register_parser().map(Parameter::Register),
            expression_parser().map(Parameter::Expression),
            indirect.map(|i| Parameter::Indirect(Box::new(i))),
        ));
    });

    let modifier = just('.').ignore_then(choice((
        alu_modifier_parser().map(Modifier::AluModifier),
        condition_parser().map(Modifier::Condition),
    )));

    let full_mnemonic = mnemonic_parser()
        .map_with_span(Spanned::new)
        .then(
            modifier
                .map_with_span(Spanned::new)
                .repeated()
                .map_with_span(Spanned::new),
        )
        .map(|(mnemonic, modifiers)| FullMnemonic {
            mnemonic,
            modifiers,
        });

    return full_mnemonic
        .map_with_span(Spanned::new)
        .padded()
        .then(
            parameter
                .map_with_span(Spanned::new)
                .padded()
                .separated_by(just(','))
                .map_with_span(Spanned::new),
        )
        .map(|(full_mnemonic, parameters)| Operation {
            full_mnemonic,
            parameters,
        });
}

fn statement_parser() -> impl Parser<char, Statement, Error = Error> {
    let label = text::ident();

    let literal = choice((
        expression_parser().map(Literal::Expression),
        string_parser().map(Literal::String),
    ));

    let assignment = text::ident()
        .map_with_span(Spanned::new)
        .then_ignore(just('=').padded())
        .then(expression_parser().map_with_span(Spanned::new));

    return choice((
        operation_parser()
            .then_ignore(just(';'))
            .map(Statement::Operation),
        label.then_ignore(just(':')).map(Statement::Label),
        assignment
            .then_ignore(just(';'))
            .map(|(ident, expr)| Statement::Assignment(ident, expr)),
        literal.then_ignore(just(';')).map(Statement::Literal),
        comment_parser().map(Statement::Comment),
    ))
    .padded();
}
