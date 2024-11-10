use std::io::Write;

use crate::Span;
use internment::Intern;

#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    pub fn write(
        &self,
        cache: impl ariadne::Cache<Intern<String>>,
        writer: impl Write,
    ) -> Result<(), std::io::Error> {
        use ariadne::{Label, Report, ReportKind};
        return Report::build(ReportKind::Error, self.span.src(), self.span.start())
            .with_code(1)
            .with_message("Assembler Error")
            .with_label(
                Label::new((self.span.src(), self.span.range())).with_message(&self.message),
            )
            .finish()
            .write(cache, writer);
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
        let message: String = format!(
            "Expected one of {}, but found {}",
            expected
                .into_iter()
                .filter_map(|e| e)
                .map(|e| format!("'{}'", e))
                .collect::<Vec<_>>()
                .join("or "),
            found
                .map(|e| format!("'{}'", e))
                .unwrap_or("nothing".to_string())
        );
        Self { message, span }
    }
    fn with_label(self, _label: Self::Label) -> Self {
        return self;
    }

    fn merge(self, _other: Self) -> Self {
        return self;
    }
}
