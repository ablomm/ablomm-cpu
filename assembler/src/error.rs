use std::io::Write;

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

impl chumsky::Error<char> for Error {
    type Span = Span;
    type Label = String;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        let label = format!(
            "Expected {}, but found {}",
            expected
                .into_iter()
                .flatten()
                .map(|e| format!("'{}'", e.escape_default().fg(ATTENTION_COLOR)))
                .collect::<Vec<_>>()
                .join("or "),
            found
                .map(|e| format!("'{}'", e.escape_default().fg(ATTENTION_COLOR)))
                .unwrap_or("nothing".to_string())
        );

        Self::new(span, "Parsing Error").with_label(label)
    }

    fn with_label(self, label: Self::Label) -> Self {
        self.with_note(label)
    }

    fn merge(mut self, other: Self) -> Self {
        for (span, label) in other.labels {
            self = self.with_label_span(span, label);
        }
        self
    }
}
