use crate::Span;
use internment::Intern;

#[derive(Debug)]
pub enum ErrorType {
    ExpectedFound {
        expected: Vec<Option<char>>,
        found: Option<char>,
    },
}

#[derive(Debug)]
pub struct Error {
    r#type: ErrorType,
    span: Span,
}

impl Error {
    pub fn print(&self, cache: impl ariadne::Cache<Intern<String>>) {
        use ariadne::{Label, Report, ReportKind};
        match &self.r#type {
            ErrorType::ExpectedFound { expected, found } => {
                let message: String = format!(
                    "Expected one of: {} but found: {}",
                    expected
                        .into_iter()
                        .filter_map(|e| e.as_ref())
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    found.unwrap_or_default()
                );

                Report::build(ReportKind::Error, self.span.src(), self.span.start())
                    .with_code(1)
                    .with_message("Unexpected Input")
                    .with_label(
                        Label::new((self.span.src(), self.span.range())).with_message(message),
                    )
                    .finish()
                    .print(cache)
                    .unwrap(); // todo: error handling
            }
        }
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
        Self {
            r#type: ErrorType::ExpectedFound {
                expected: expected.into_iter().collect(),
                found,
            },
            span,
        }
    }
    fn with_label(self, _label: Self::Label) -> Self {
        return self;
    }

    fn merge(self, _other: Self) -> Self {
        return self;
    }
}
