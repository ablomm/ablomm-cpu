use std::{fmt::Display, io::Write};

use crate::{src::Src, Span};
use ariadne::{Color, Fmt};
use internment::Intern;

pub const ATTENTION_COLOR: Color = Color::Fixed(12); // blue

#[derive(Debug)]
pub struct Error {
    span: Span, // main span
    message: String,
    labels: Vec<(Span, String)>,
    notes: Vec<String>,
    help: Option<String>,
}

impl Error {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Error {
            span,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
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
        self.help = Some(help.into());
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

        // ariadne for some reason does not reutrn self in this call, so needs to be seperate from
        // the previous
        report.with_notes(self.notes.clone());

        if let Some(help) = &self.help {
            report = report.with_help(help);
        }

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
impl Error {
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

        Error::new(span, format!("Incorrect number of {object_name}s")).with_label(format!(
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
            "nothing".to_string()
        } else {
            expected
                .into_iter()
                .map(|val| val.fg(ATTENTION_COLOR).to_string())
                .collect::<Vec<_>>()
                .join(" or ")
        };

        // "nothing" has no highlights, so it's clear it is not a value called "nothing"
        let found = found.map_or("nothing".to_string(), |found| {
            found.fg(ATTENTION_COLOR).to_string()
        });

        Error::new(span, format!("Incorrect {object_name}"))
            .with_label(format!("Expected {}, but found {}", expected, found,))
    }

    pub fn identifier_already_defined(first_define: Span, second_define: Span) -> Self {
        Error::new(second_define, "Identifier already defined")
            .with_label_span(first_define, "Defined first here")
            .with_label("Defined again here")
    }
}

// this isnt used right now because chumsky calls merge() multiple times for the same span which
// does not work well with the structure of Error, so instead, we are using Chumsky's error::Simple
// which will have merged everything, then just convert it into an Error
impl chumsky::Error<char> for Error {
    type Span = Span;
    type Label = String;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        Self::incorrect_value(
            span,
            "token",
            expected
                .into_iter()
                .flatten()
                .map(|c| format!("'{}'", c.escape_default()))
                .collect::<Vec<_>>(),
            found.map(|c| format!("'{}'", c.escape_default())),
        )
    }

    fn with_label(self, label: Self::Label) -> Self {
        self.with_note(label)
    }

    // a bit broken since chumsky will call this function even on the same span
    fn merge(mut self, other: Self) -> Self {
        for (span, label) in other.labels {
            self = self.with_label_span(span, label);
        }
        self
    }
}

// workaround for chumsky calling merge() multiple times on the same span: simply convert
// error::Simple to Error after everything is merged by error::Simple
impl From<chumsky::error::Simple<char, Span>> for Error {
    fn from(value: chumsky::error::Simple<char, Span>) -> Self {
        Self::incorrect_value(
            value.span(),
            "token",
            value
                .expected()
                .flatten()
                .map(|c| format!("'{}'", c.escape_default()))
                .collect(),
            value.found().map(|c| format!("'{}'", c.escape_default())),
        )
    }
}
