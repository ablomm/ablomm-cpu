use std::{
    cmp::Ordering,
    fmt::Display,
    io::{self, Write},
};

use crate::{
    Span, error::ATTENTION_COLOR, expression::expression_result::ExpressionResult, span::Spanned,
    src::Src, utils,
};
use ariadne::{Cache, Color, Fmt};
use chumsky::error::{RichPattern, RichReason};
use internment::Intern;

#[derive(Debug)]
pub struct SpannedError {
    pub span: Span, // main span
    pub message: String,
    pub labels: Vec<(Span, String)>,
    pub notes: Vec<String>,
    pub helps: Vec<String>,
}

impl SpannedError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        SpannedError {
            span,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
        }
    }

    // will just use the main span
    pub fn with_label(mut self, message: impl Into<String>) -> Self {
        self.labels.push((self.span, message.into()));
        self
    }

    // to use a new span
    pub fn with_label_span(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into()));
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.helps.push(help.into());
        self
    }

    // check if any of the spans in this error intersect the given span
    #[allow(dead_code)]
    pub(crate) fn any_span_intersect(&self, span: Span) -> bool {
        span.intersects(&self.span)
            || self
                .labels
                .iter()
                .any(|(label_span, _)| span.intersects(label_span))
    }

    pub fn write(
        &self,
        writer: impl Write,
        cache: impl Cache<Intern<Src>>,
    ) -> Result<(), std::io::Error> {
        use ariadne::{Config, IndexType, Label, Report, ReportKind};

        let labels = self.labels.iter().map(|(span, message)| {
            Label::new(*span)
                .with_message(message)
                .with_color(Color::Fixed(9))
        });

        let mut report = Report::build(ReportKind::Error, self.span)
            .with_message(&self.message)
            .with_labels(labels);

        // ariadne for some reason does not return self in this call, so needs to be separate from
        // the previous
        report.with_notes(self.notes.clone());

        report.with_helps(self.helps.clone());

        report
            .with_config(Config::new().with_index_type(IndexType::Byte))
            .finish()
            .write(cache, writer)
    }

    pub fn eprint(&self, cache: impl ariadne::Cache<Intern<Src>>) -> Result<(), std::io::Error> {
        self.write(io::stderr(), cache)
    }

    pub fn print(&self, cache: impl ariadne::Cache<Intern<Src>>) -> Result<(), std::io::Error> {
        self.write(io::stdout(), cache)
    }
}

// all the specific error type constructors
impl SpannedError {
    pub(crate) fn incorrect_num(
        span: Span,
        object_name: impl Display,
        expected: Vec<usize>,
        found: usize,
    ) -> Self {
        // make plural if it is more than one objects
        let object_name_pluralized = if expected.len() == 1 && expected[0] == 1 {
            object_name.to_string()
        } else {
            object_name.to_string() + "s"
        };

        SpannedError::new(span, format!("Incorrect number of {object_name}s")).with_label(format!(
            "Expected {} {object_name_pluralized}, but found {}",
            expected
                .into_iter()
                .map(|val| format!("{}", val.fg(ATTENTION_COLOR)))
                .collect::<Vec<_>>()
                .join(" or "),
            found.fg(ATTENTION_COLOR)
        ))
    }

    pub(crate) fn incorrect_type(
        expected: Vec<impl Display>,
        found: &Spanned<&ExpressionResult>,
    ) -> Self {
        Self::incorrect_value(found.span, "type", expected, Some(found.val))
    }

    pub(crate) fn incorrect_value(
        span: Span,
        object_name: impl Display,
        expected: Vec<impl Display>,
        found: Option<impl Display>,
    ) -> Self {
        let expected = if expected.is_empty() {
            "nothing".fg(ATTENTION_COLOR).to_string()
        } else {
            expected
                .into_iter()
                .map(|val| val.fg(ATTENTION_COLOR).to_string())
                .collect::<Vec<_>>()
                .join(" or ")
        };

        let found = found.map_or("nothing".fg(ATTENTION_COLOR).to_string(), |found| {
            found.fg(ATTENTION_COLOR).to_string()
        });

        SpannedError::new(span, format!("Incorrect {object_name}"))
            .with_label(format!("Expected {}, but found {}", expected, found,))
    }

    pub(crate) fn identifier_already_defined(
        define1: Span,
        define1_import: Option<Span>,
        define2: Span,
        define2_import: Option<Span>,
    ) -> Self {
        let order = utils::fallback_ordering(
            &[define1_import, Some(define1)].iter().flatten().collect(),
            &[define2_import, Some(define2)].iter().flatten().collect(),
        )
        .unwrap_or(Ordering::Less);

        let (first_define, first_import, second_define, second_import) = match order {
            Ordering::Less | Ordering::Equal => (define1, define1_import, define2, define2_import),
            Ordering::Greater => (define2, define2_import, define1, define1_import),
        };

        let mut error = SpannedError::new(second_define, "Identifier already defined")
            .with_label_span(first_define, "Defined first here");

        if let Some(import) = first_import {
            error = error.with_label_span(import, "Imported here")
        }

        error = error.with_label("Defined again here");

        if let Some(import) = second_import {
            let message = if first_import.is_some() {
                "Imported again here"
            } else {
                "Imported here"
            };

            error = error.with_label_span(import, message)
        }

        error.with_help("Try using a different name")
    }
}

impl From<&chumsky::error::Rich<'_, char, Span>> for SpannedError {
    fn from(value: &chumsky::error::Rich<'_, char, Span>) -> Self {
        match value.reason() {
            RichReason::ExpectedFound { expected, found } => Self::incorrect_value(
                *value.span(),
                "token",
                expected.iter().map(format_rich_pattern).collect(),
                found.map(|c| format!("'{}'", c.escape_default())),
            ),
            chumsky::error::RichReason::Custom(message) => {
                Self::new(*value.span(), "Parse error").with_label(message)
            }
        }
    }
}

fn format_rich_pattern<T: Display>(rich_pattern: &RichPattern<'_, T>) -> String {
    match rich_pattern {
        RichPattern::Token(token) => format!("'{}'", token.to_string().escape_default()),
        RichPattern::Label(label) => label.to_string(),
        RichPattern::Identifier(identifier) => identifier.to_string(),
        RichPattern::Any => "anything".to_string(),
        RichPattern::SomethingElse => "something else".to_string(),
        RichPattern::EndOfInput => "end of input".to_string(),
        // all cases are covered as of 0.12.0, but the enum is non_exhaustive
        _ => "unknown".to_string(),
    }
}
