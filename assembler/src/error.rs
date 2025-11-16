use std::{fmt::Display, io::Write};

use crate::{Span, src::Src};
use ariadne::{Cache, Color, Fmt};
use chumsky::error::{RichPattern, RichReason};
use internment::Intern;

pub const ATTENTION_COLOR: Color = Color::Fixed(12); // blue

pub enum Error<T: Cache<Intern<Src>>> {
    Spanned(Vec<SpannedError>, T),
    Bare(String),
}

#[derive(Debug)]
pub struct SpannedError {
    span: Span, // main span
    message: String,
    labels: Vec<(Span, String)>,
    notes: Vec<String>,
    helps: Vec<String>,
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

    pub fn write(
        &self,
        cache: impl ariadne::Cache<Intern<Src>>,
        writer: impl Write,
    ) -> Result<(), std::io::Error> {
        use ariadne::{Label, Report, ReportKind};

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

        report.finish().write(cache, writer)
    }

    pub fn eprint(&self, cache: impl ariadne::Cache<Intern<Src>>) -> Result<(), std::io::Error> {
        self.write(cache, std::io::stderr())
    }

    pub fn print(&self, cache: impl ariadne::Cache<Intern<Src>>) -> Result<(), std::io::Error> {
        self.write(cache, std::io::stdout())
    }
}

// all the specific error type constructors
impl SpannedError {
    pub fn incorrect_num(
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

    pub fn incorrect_value(
        span: Span,
        object_name: impl Display,
        expected: Vec<impl Display>,
        found: Option<impl Display>,
    ) -> Self {
        let expected = if expected.is_empty() {
            // "nothing" has no highlights, so it's clear it is not a value called "nothing"
            "nothing".fg(ATTENTION_COLOR).to_string()
        } else {
            expected
                .into_iter()
                .map(|val| val.fg(ATTENTION_COLOR).to_string())
                .collect::<Vec<_>>()
                .join(" or ")
        };

        // "nothing" has no highlights, so it's clear it is not a value called "nothing"
        let found = found.map_or("nothing".fg(ATTENTION_COLOR).to_string(), |found| {
            found.fg(ATTENTION_COLOR).to_string()
        });

        SpannedError::new(span, format!("Incorrect {object_name}"))
            .with_label(format!("Expected {}, but found {}", expected, found,))
    }

    pub fn identifier_already_defined(
        first_define: Span,
        first_define_import: Option<Span>,
        second_define: Span,
        second_define_import: Option<Span>,
    ) -> Self {
        let mut error = SpannedError::new(second_define, "Identifier already defined")
            .with_label_span(first_define, "Defined first here");

        if let Some(import) = first_define_import {
            error = error.with_label_span(import, "Imported here")
        }

        error = error.with_label("Defined again here");

        if let Some(import) = second_define_import {
            let message = if first_define_import.is_some() {
                "Imported again here"
            } else {
                "Imported here"
            };

            error = error.with_label_span(import, message)
        }

        error.with_help("Try using a different name")
    }
}

// workaround for chumsky calling merge() multiple times on the same span: simply convert
// error::Rich to SpannedError after everything is merged by error::Rich
impl From<chumsky::error::Rich<'_, char, Span>> for SpannedError {
    fn from(value: chumsky::error::Rich<'_, char, Span>) -> Self {
        match value.reason() {
            RichReason::ExpectedFound { expected, found } => Self::incorrect_value(
                *value.span(),
                "token",
                expected.iter().map(format_rich_pattern).collect(),
                found.map(|c| format!("'{}'", c.escape_default())),
            ),
            chumsky::error::RichReason::Custom(message) => {
                Self::new(*value.span(), message).with_label(message)
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
    }
}
