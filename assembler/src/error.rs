use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::{Span, expression::expression_result::ExpressionResult, span::Spanned, src::Src};
use ariadne::{Color, Fmt};
use internment::Intern;

pub mod spanned;

pub use spanned::SpannedError;

pub const ATTENTION_COLOR: Color = Color::Blue;

// if there was errors, but we were able to recover some result
pub struct RecoveredError<T, E = Vec<SpannedError>>(pub T, pub E);
pub type RecoveredResult<T, Te = T, E = Vec<Error>> = Result<T, RecoveredError<Te, E>>;

pub enum Error {
    Bare(String),

    // boxed because it's large
    Spanned(Box<SpannedError>),

    // will not print anything
    // useful if this error is a consequence of a previous error that should already be printed, so
    // printing this one would just increase noise
    Silenced(Box<Error>),
}

impl Error {
    pub fn write(
        &self,
        cache: impl ariadne::Cache<Intern<Src>>,
        mut writer: impl Write,
    ) -> Result<(), std::io::Error> {
        match self {
            Self::Bare(error) => writeln!(writer, "{} {}", "Error:".fg(Color::Red), error),
            Self::Spanned(error) => error.write(writer, cache),
            Self::Silenced(_) => Ok(()),
        }
    }

    pub fn eprint(&self, cache: impl ariadne::Cache<Intern<Src>>) -> Result<(), std::io::Error> {
        self.write(cache, io::stderr())
    }

    pub fn print(&self, cache: impl ariadne::Cache<Intern<Src>>) -> Result<(), std::io::Error> {
        self.write(cache, io::stdout())
    }

    // add help if the error type supports it
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        #[allow(clippy::single_match)] // may add more sub-types with helps
        match &mut self {
            Self::Spanned(spanned_error) => {
                spanned_error.helps.push(help.into());
            }
            _ => (),
        }
        self
    }

    // add note if the error type supports it
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        #[allow(clippy::single_match)] // may add more sub-types with notes
        match &mut self {
            Self::Spanned(spanned_error) => {
                spanned_error.notes.push(note.into());
            }
            _ => (),
        }
        self
    }
}

impl Error {
    pub fn incorrect_num(
        span: Span,
        object_name: impl Display,
        expected: Vec<usize>,
        found: usize,
    ) -> Self {
        Self::Spanned(Box::new(SpannedError::incorrect_num(
            span,
            object_name,
            expected,
            found,
        )))
    }

    pub fn incorrect_type(expected: Vec<impl Display>, found: &Spanned<&ExpressionResult>) -> Self {
        let error = Self::Spanned(Box::new(SpannedError::incorrect_type(expected, found)));
        // Error type is ignored because there should already be an error for when that result was
        // created; no need to print any more errors about it
        if matches!(found.val, ExpressionResult::Error) {
            Self::Silenced(Box::new(error))
        } else {
            error
        }
    }

    pub fn incorrect_value(
        span: Span,
        object_name: impl Display,
        expected: Vec<impl Display>,
        found: Option<impl Display>,
    ) -> Self {
        Self::Spanned(Box::new(SpannedError::incorrect_value(
            span,
            object_name,
            expected,
            found,
        )))
    }

    pub fn identifier_already_defined(
        first_define: Span,
        first_define_import: Option<Span>,
        second_define: Span,
        second_define_import: Option<Span>,
    ) -> Self {
        Self::Spanned(Box::new(SpannedError::identifier_already_defined(
            first_define,
            first_define_import,
            second_define,
            second_define_import,
        )))
    }
}

impl<T: Into<String>> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Bare(value.into())
    }
}

impl From<SpannedError> for Error {
    fn from(value: SpannedError) -> Self {
        Self::Spanned(Box::new(value))
    }
}
