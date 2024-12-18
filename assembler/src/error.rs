use std::io::Write;

use crate::{src::Src, Span};
use ariadne::{Color, Fmt};
use internment::Intern;

pub const ATTENTION_COLOR: Color = Color::Fixed(12); // blue

#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
    note: Option<String>,
}

impl Error {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn write(
        &self,
        cache: impl ariadne::Cache<Intern<Src>>,
        writer: impl Write,
    ) -> Result<(), std::io::Error> {
        use ariadne::{Label, Report, ReportKind};
        let mut report = Report::build(ReportKind::Error, self.span)
            .with_code(1)
            .with_message("Assembler Error")
            .with_label(
                Label::new(self.span)
                    .with_message(&self.message)
                    .with_color(Color::Fixed(9)), // red
            );

        if let Some(note) = &self.note {
            report = report.with_note(note);
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
    type Label = ();

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        let message = format!(
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
        Self {
            message,
            span,
            note: None,
        }
    }

    // not implemented
    fn with_label(self, _label: Self::Label) -> Self {
        self
    }

    // not implemented
    fn merge(self, _other: Self) -> Self {
        self
    }
}
