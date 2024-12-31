use std::{
    fmt::Display,
    ops::{Add, BitAnd, BitOr, BitXor, Deref, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use super::*;

mod indirect;
mod number;
mod register;
mod register_offset;
mod string;

// the Option<T> is used to allow evaluating expressions in order to get it's type, without knowing
// the concrete value; useful for calculating the number of words of some statements
#[derive(Debug, Clone)]
pub enum ExpressionResult {
    Number(Option<Number>),
    String(Option<String>),
    Register(Option<Register>),
    RegisterOffset(Option<RegisterOffset>),
    Indirect(Indirect),
}

// newtypes
#[derive(Debug, Clone, Copy)]
pub struct Number(pub u32);
impl Deref for Number {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct String(pub std::string::String);

impl Deref for String {
    type Target = std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RegisterOffset {
    pub reg: Spanned<Register>,
    pub offset: i32,
}

#[derive(Debug, Clone)]
pub struct Indirect(pub Box<ExpressionResult>);

impl Deref for Indirect {
    type Target = Box<ExpressionResult>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ExpressionResult::Number(_) => "number",
            ExpressionResult::String(_) => "string",
            ExpressionResult::Register(_) => "register",
            ExpressionResult::RegisterOffset(_) => "register offset",
            ExpressionResult::Indirect(indirect) => &format!("{} {}", "indirect", **indirect),
        };

        write!(f, "{}", string)
    }
}

// operations that are not in rust std::op

// asmref to make it clear this is different from rust ref
pub trait AsmRef {
    type Output;

    fn asm_ref(self) -> Self::Output;
}

// asmref to make it clear this is different from rust deref
pub trait AsmDeref {
    type Output;

    fn asm_deref(self) -> Self::Output;
}

pub trait Ashr<Rhs = Self> {
    type Output;

    fn ashr(self, rhs: Rhs) -> Self::Output;
}

impl AsmRef for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;

    fn asm_ref(self) -> Self::Output {
        match self.val {
            ExpressionResult::Indirect(indirect) => self.span_to(indirect).asm_ref(),
            _ => Err(Error::new(self.span, "Incorrect type")
                .with_label(format!(
                    "Expected {}, but found {}",
                    "indirect".fg(ATTENTION_COLOR),
                    self.fg(ATTENTION_COLOR)
                ))
                .with_note("You can only take a reference of a value was previously dereferenced")),
        }
    }
}

impl AsmDeref for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;

    fn asm_deref(self) -> Self::Output {
        Ok(ExpressionResult::Indirect(Indirect(Box::new(
            self.val.clone(),
        ))))
    }
}

impl Neg for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn neg(self) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => -&self.span_to(number),
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Not for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn not(self) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => !&self.span_to(number),
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Mul<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn mul(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) * rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Div<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn div(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) / rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Rem<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn rem(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) % rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) + rhs,
            ExpressionResult::String(string) => &self.span_to(string) + rhs,
            ExpressionResult::Register(register) => &self.span_to(register) + rhs,
            ExpressionResult::RegisterOffset(reg_offset) => &self.span_to(reg_offset) + rhs,
            _ => Err(Error::new(rhs.span, "Incorrect type").with_label(format!(
                "Expected {}, {}, {}, or {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                "string".fg(ATTENTION_COLOR),
                "register".fg(ATTENTION_COLOR),
                "register offset".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) - rhs,
            ExpressionResult::Register(register) => &self.span_to(register) - rhs,
            ExpressionResult::RegisterOffset(reg_offset) => &self.span_to(reg_offset) - rhs,
            _ => Err(Error::new(rhs.span, "Incorrect type").with_label(format!(
                "Expected {}, {}, or {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                "register".fg(ATTENTION_COLOR),
                "register offset".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Shl<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn shl(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) << rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Shr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn shr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) >> rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Ashr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn ashr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => self.span_to(number).ashr(rhs),
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl BitAnd<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn bitand(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) & rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl BitOr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn bitor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) | rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl BitXor<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn bitxor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) ^ rhs,
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                self.fg(ATTENTION_COLOR)
            ))),
        }
    }
}
