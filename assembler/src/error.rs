use std::io::Write;

use crate::Span;
use internment::Intern;

#[derive(Debug)]
pub enum ErrorType {
    ExpectedFound {
        expected: Vec<String>,
        found: Option<String>,
        span: Span,
    },
}

#[derive(Debug)]
pub struct Error {
    r#type: ErrorType,
    message: String,
}

impl Error {
    pub fn write(
        &self,
        cache: impl ariadne::Cache<Intern<String>>,
        writer: impl Write,
    ) -> Result<(), std::io::Error> {
        use ariadne::{Label, Report, ReportKind};
        match &self.r#type {
            ErrorType::ExpectedFound {
                expected: _expected,
                found: _found,
                span,
            } => {
                return Report::build(ReportKind::Error, span.src(), span.start())
                    .with_code(1)
                    .with_message("Unexpected Input")
                    .with_label(Label::new((span.src(), span.range())).with_message(&self.message))
                    .finish()
                    .write(cache, writer)
            }
        }
    }

    pub fn eprint(&self, cache: impl ariadne::Cache<Intern<String>>) -> Result<(), std::io::Error> {
        return self.write(cache, std::io::stderr());
    }

    pub fn print(&self, cache: impl ariadne::Cache<Intern<String>>) -> Result<(), std::io::Error> {
        return self.write(cache, std::io::stdout());
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
        let expected: Vec<_> = expected
            .into_iter()
            .filter_map(|e| e)
            .map(|e| e.to_string())
            .collect();
        let found = found.map(|e| e.to_string());
        let message: String = format!(
            "Expected one of {}, but found {}",
            expected
                .iter()
                .map(|e| format!("'{}'", e))
                .collect::<Vec<_>>()
                .join("or "),
            found
                .clone()
                .map(|e| format!("'{}'", e))
                .unwrap_or("nothing".to_string())
        );
        Self {
            r#type: ErrorType::ExpectedFound {
                expected,
                found,
                span,
            },
            message,
        }
    }
    fn with_label(self, _label: Self::Label) -> Self {
        return self;
    }

    fn merge(self, _other: Self) -> Self {
        return self;
    }
}
