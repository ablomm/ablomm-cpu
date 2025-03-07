use crate::symbol_table::SymbolTable;
use crate::{ast::*, Span};
use chumsky::prelude::*;
use internment::Intern;
use std::cell::RefCell;
use std::char;
use std::collections::HashMap;
use std::rc::Rc;
use text::TextParser;

mod expression;
mod keywords;

pub type ParseError = Simple<char, Span>;

pub fn block_parser() -> impl Parser<char, Spanned<Block>, Error = ParseError> {
    statement_parser()
        .map_with_span(Spanned::new)
        .padded()
        .repeated()
        .then_ignore(end())
        .map(|statements| Block {
            statements,
            symbol_table: Rc::new(RefCell::new(SymbolTable {
                table: HashMap::new(),
                parent: None,
            })),
        })
        .map_with_span(Spanned::new)
}

fn comment_parser() -> impl Parser<char, String, Error = ParseError> {
    let line_comment = just("//").ignore_then(take_until(just("\n")));
    let multiline_comment = just("/*").ignore_then(take_until(just("*/")));

    line_comment
        .or(multiline_comment)
        .map(|(_, comment)| comment.into())
}

fn string_parser() -> impl Parser<char, String, Error = ParseError> {
    let escape_string = just('\\').ignore_then(choice((
        just('\\').to('\\'),
        just('\"').to('"'),
        just('0').to('\0'),
        just('t').to('\t'),
        just('n').to('\n'),
        just('r').to('\r'),
    )));

    filter(|c| *c != '\\' && *c != '"')
        .or(escape_string)
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
}

fn operation_parser() -> impl Parser<char, Operation, Error = ParseError> {
    let modifier = just('.').ignore_then(choice((
        keywords::alu_modifier_parser().map(Modifier::AluModifier),
        keywords::condition_parser().map(Modifier::Condition),
    )));

    let full_mnemonic = keywords::mnemonic_parser()
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

    full_mnemonic
        .map_with_span(Spanned::new)
        .padded()
        .then(
            expression::expression_parser()
                .map_with_span(Spanned::new)
                .padded()
                .separated_by(just(','))
                .map_with_span(Spanned::new),
        )
        .map(|(full_mnemonic, operands)| Operation {
            full_mnemonic,
            operands,
        })
}

fn statement_parser() -> impl Parser<char, Statement, Error = ParseError> {
    let label = just("export")
        .padded()
        .or_not()
        .then(text::ident().map(Intern::new).map_with_span(Spanned::new))
        .map(|(export, identifier)| Label {
            export: export.is_some(),
            identifier,
        });

    let assignment = just("export")
        .padded()
        .or_not()
        .then(text::ident().map(Intern::new).map_with_span(Spanned::new))
        .then_ignore(just('=').padded())
        .then(expression::expression_parser().map_with_span(Spanned::new))
        .map(|((export, identifier), expression)| Assignment {
            export: export.is_some(),
            identifier,
            expression,
        });

    let export = just("export").ignore_then(
        text::ident()
            .map(Intern::new)
            .map_with_span(Spanned::new)
            .padded()
            .separated_by(just(',')),
    );

    recursive(|statement| {
        let block = statement
            .map_with_span(Spanned::new)
            .padded()
            .repeated()
            .padded() // if there is no statements in the block
            .delimited_by(just('{'), just('}'))
            .map(|statements| Block {
                statements,
                symbol_table: Rc::new(RefCell::new(SymbolTable {
                    table: HashMap::new(),
                    parent: None,
                })),
            });

        choice((
            block.padded().map(Statement::Block),
            operation_parser()
                .then_ignore(just(';'))
                .map(Statement::Operation),
            label.padded().then_ignore(just(':')).map(Statement::Label),
            assignment
                .padded()
                .then_ignore(just(';'))
                .map(Statement::Assignment),
            expression::expression_parser()
                .padded()
                .then_ignore(just(';'))
                .map(Statement::GenLiteral),
            export
                .padded()
                .then_ignore(just(';'))
                .map(Statement::Export),
            import_parser()
                .padded()
                .then_ignore(just(';'))
                .map(Statement::Import),
            comment_parser().padded().map(Statement::Comment),
        ))
    })
}

fn import_parser() -> impl Parser<char, Import, Error = ParseError> {
    let named_import = text::ident()
        .map(Intern::new)
        .map_with_span(Spanned::new)
        .padded()
        .then(
            just("as")
                .ignore_then(
                    text::ident()
                        .map(Intern::new)
                        .map_with_span(Spanned::new)
                        .padded(),
                )
                .or_not(),
        )
        .map(|(identifier, alias)| NamedImport { identifier, alias });

    just("import")
        .ignore_then(choice((
            just("*")
                .to(ImportSpecifier::Blob)
                .map_with_span(Spanned::new)
                .padded(),
            named_import
                .map_with_span(Spanned::new)
                .separated_by(just(','))
                .map(ImportSpecifier::Named)
                .map_with_span(Spanned::new)
                .padded(),
        )))
        .then(just("from").ignore_then(string_parser().map_with_span(Spanned::new).padded()))
        .map(|(specifier, file)| Import { file, specifier })
}
