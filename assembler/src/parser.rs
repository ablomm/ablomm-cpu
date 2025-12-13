use crate::Span;
use crate::ast::{
    Assignment, Block, File, FullMnemonic, Import, ImportSpecifier, Label, Modifier, NamedImport,
    Operation, Statement,
};
use crate::span::Spanned;
use crate::symbol_table::SymbolTable;
use chumsky::input::StrInput;
use chumsky::prelude::*;
use internment::Intern;
use std::cell::RefCell;
use std::char;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

mod expression;
mod keywords;

type ParseError<'src> = Rich<'src, char, Span>;

type Extra<'src> = extra::Err<ParseError<'src>>;

// a trait alias for StrInput to make parsers more concise
pub(super) trait Input<'src>:
    StrInput<'src, Token = char, Span = Span, Slice = &'src str>
{
}

impl<'src, T> Input<'src> for T where T: StrInput<'src, Token = char, Span = Span, Slice = &'src str>
{}

pub(super) fn file_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, File, Extra<'src>> {
    statement_parser()
        .map_with(|val, e| Spanned::new(val, e.span()))
        .padded_by(comment_pad())
        .repeated()
        .collect::<Vec<_>>()
        .map(|statements| Block {
            statements,
            symbol_table: Rc::new(RefCell::new(SymbolTable {
                table: HashMap::new(),
                parent: None,
            })),
        })
        .map(|block| File { block })
        .labelled("file block")
}

fn statement_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, Statement, Extra<'src>> {
    let label = text::keyword("export")
        .or_not()
        .then(
            text::ident()
                .map(|s: &str| Intern::new(s.to_string()))
                .map_with(|val, e| Spanned::new(val, e.span()))
                .padded_by(comment_pad()),
        )
        .map(|(export, identifier)| Label {
            export: export.is_some(),
            identifier,
        })
        .labelled("label");

    let assignment = text::keyword("export")
        .or_not()
        .then(
            text::ident()
                .map(|s: &str| Intern::new(s.to_string()))
                .map_with(|val, e| Spanned::new(val, e.span()))
                .padded_by(comment_pad()),
        )
        .then_ignore(just('=').padded_by(comment_pad()))
        .then(expression::expression_parser().map_with(|val, e| Spanned::new(val, e.span())))
        .map(|((export, identifier), expression)| Assignment {
            export: export.is_some(),
            identifier,
            expression,
        })
        .labelled("assignment");

    let export = text::keyword("export")
        .ignore_then(
            text::ident()
                .map(|s: &str| Intern::new(s.to_string()))
                .map_with(|val, e| Spanned::new(val, e.span()))
                .padded_by(comment_pad())
                .separated_by(just(','))
                .collect::<Vec<_>>(),
        )
        .labelled("export");

    recursive(|statement| {
        let block = statement
            .map_with(|val, e| Spanned::new(val, e.span()))
            .padded_by(comment_pad())
            .repeated()
            .collect::<Vec<_>>()
            .padded_by(comment_pad()) // if there is no statements in the block
            .delimited_by(just('{'), just('}'))
            .map(|statements| Block {
                statements,
                symbol_table: Rc::new(RefCell::new(SymbolTable {
                    table: HashMap::new(),
                    parent: None,
                })),
            })
            .labelled("block");

        choice((
            block.map(Statement::Block),
            operation_parser()
                .padded_by(comment_pad())
                .then_ignore(just(';'))
                .map(Statement::Operation),
            label
                .padded_by(comment_pad())
                .then_ignore(just(':'))
                .map(Statement::Label),
            assignment
                .padded_by(comment_pad())
                .then_ignore(just(';'))
                .map(Statement::Assignment),
            expression::expression_parser()
                .padded_by(comment_pad())
                .then_ignore(just(';'))
                .map(Statement::GenLiteral),
            export
                .padded_by(comment_pad())
                .then_ignore(just(';'))
                .map(Statement::Export),
            import_parser()
                .padded_by(comment_pad())
                .then_ignore(just(';'))
                .map(Statement::Import),
        ))
        .recover_with(skip_until(
            // once it reaches '}' (or end()), then this statement couldn't recover anything.
            // If this just('}').not() wasn't present, then the parser would skip over a '}'
            // just to find a statement from another scope
            any().and_is(just('}').not()).ignored(),
            choice((just(';'), just(':'))).ignored(),
            || Statement::Error,
        ))
        .labelled("statement")
        .boxed()
    })
}

fn operation_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, Operation, Extra<'src>> {
    let modifier = just('.')
        .ignore_then(choice((
            keywords::alu_modifier_parser().map(Modifier::AluModifier),
            keywords::condition_parser().map(Modifier::Condition),
        )))
        .labelled("modifier");

    let full_mnemonic = keywords::mnemonic_parser()
        .map_with(|val, e| Spanned::new(val, e.span()))
        .then(
            modifier
                .map_with(|val, e| Spanned::new(val, e.span()))
                .repeated()
                .collect::<Vec<_>>()
                .map_with(|val, e| Spanned::new(val, e.span())),
        )
        .map(|(mnemonic, modifiers)| FullMnemonic {
            mnemonic,
            modifiers,
        });

    full_mnemonic
        .map_with(|val, e| Spanned::new(val, e.span()))
        .padded_by(comment_pad())
        .then(
            expression::expression_parser()
                .map_with(|val, e| Spanned::new(val, e.span()))
                .padded_by(comment_pad())
                .separated_by(just(','))
                .collect::<Vec<_>>()
                .map_with(|val, e| Spanned::new(val, e.span())),
        )
        .map(|(full_mnemonic, operands)| Operation {
            full_mnemonic,
            operands,
        })
        .labelled("operation")
}

fn import_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, Import, Extra<'src>> {
    let named_import = text::ident()
        .map(|s: &str| Intern::new(s.to_string()))
        .map_with(|val, e| Spanned::new(val, e.span()))
        .then(
            text::keyword("as")
                .padded_by(comment_pad())
                .ignore_then(
                    text::ident()
                        .map(|s: &str| Intern::new(s.to_string()))
                        .map_with(|val, e| Spanned::new(val, e.span())),
                )
                .or_not(),
        )
        .map(|(identifier, alias)| NamedImport { identifier, alias });

    text::keyword("import")
        .ignore_then(
            choice((
                just("*").to(ImportSpecifier::Glob),
                named_import
                    .map_with(|val, e| Spanned::new(val, e.span()))
                    .padded_by(comment_pad())
                    .separated_by(just(','))
                    .collect::<Vec<_>>()
                    .map(ImportSpecifier::Named),
            ))
            .map_with(|val, e| Spanned::new(val, e.span()))
            .padded_by(comment_pad()),
        )
        .then(text::keyword("from").padded_by(comment_pad()).ignore_then(
            expression::string_parser().try_map(|import_string, span: Span| {
                // there seems to be a bug in chumsky where this span is being changed to a single character
                // instead of using the full string span
                let import_src = span
                    .src
                    .get_relative(Path::new(&import_string))
                    .map_err(|error| ParseError::custom(span, error))?;
                Ok(Spanned::new(Intern::new(import_src), span))
            }),
        ))
        .map(|(specifier, src)| Import { src, specifier })
        .labelled("import")
}

fn comment_parser<'src, I: Input<'src>>() -> impl Parser<'src, I, String, Extra<'src>> {
    let line_comment = any()
        .and_is(text::newline().not())
        .repeated()
        .collect::<String>()
        .delimited_by(just("//"), text::newline())
        .labelled("single-line comment");

    // nested multi-line comments!
    let multiline_comment = recursive(|multiline_comment| {
        choice((
            // for nested comments, give back the /**/ (because they are part of the text of the parent comment)
            multiline_comment.map(|comment| format!("/*{}*/", comment).to_string()),
            // a string up until the next nested comment
            any()
                .and_is(just("/*").or(just("*/")).not())
                .repeated()
                .at_least(1)
                .collect::<String>(),
        ))
        .repeated()
        .collect::<Vec<_>>()
        .map(|strings| strings.join(""))
        .delimited_by(just("/*"), just("*/"))
        .labelled("multi-line comment")
    });

    line_comment.or(multiline_comment).labelled("comment")
}

fn comment_pad<'src, I: Input<'src>>() -> impl Parser<'src, I, (), Extra<'src>> {
    comment_parser().padded().repeated().padded().ignored()
}
