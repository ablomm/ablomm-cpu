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
    // TODO: skip printing errors caused by the Error type, since the root cause is already printed earlier
    Error,
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

impl Spanned<&Number> {
    pub fn as_u32(&self) -> Spanned<&u32> {
        self.span_to(**self)
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

impl ExpressionResult {
    pub fn is_known_val(&self) -> bool {
        match self {
            ExpressionResult::Number(None) => false,
            ExpressionResult::String(None) => false,
            ExpressionResult::Register(None) => false,
            ExpressionResult::RegisterOffset(None) => false,
            ExpressionResult::Indirect(indirect) => indirect.is_known_val(),
            _ => true,
        }
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
            ExpressionResult::Error => "unknown (from previous error)",
        };

        write!(f, "{}", string)
    }
}

impl From<ExpressionResult> for std::string::String {
    fn from(value: ExpressionResult) -> Self {
        value.to_string()
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
    type Output = Result<ExpressionResult, SpannedError>;

    fn asm_ref(self) -> Self::Output {
        match self.val {
            ExpressionResult::Indirect(indirect) => self.span_to(indirect).asm_ref(),
            _ => Err(
                SpannedError::incorrect_type(vec!["indirect"], self).with_note(
                    "You can only take a reference of a value that was previously dereferenced",
                ),
            ),
        }
    }
}

impl AsmDeref for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;

    fn asm_deref(self) -> Self::Output {
        Ok(ExpressionResult::Indirect(Indirect(Box::new(
            self.val.clone(),
        ))))
    }
}

impl Neg for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn neg(self) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => -&self.span_to(number),
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Not for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn not(self) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => !&self.span_to(number),
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Mul<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn mul(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) * rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Div<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn div(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) / rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Rem<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn rem(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) % rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) + rhs,
            ExpressionResult::String(string) => &self.span_to(string) + rhs,
            ExpressionResult::Register(register) => &self.span_to(register) + rhs,
            ExpressionResult::RegisterOffset(reg_offset) => &self.span_to(reg_offset) + rhs,
            _ => Err(SpannedError::incorrect_type(
                vec!["number", "string", "register", "register offset"],
                self,
            )),
        }
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) - rhs,
            ExpressionResult::Register(register) => &self.span_to(register) - rhs,
            ExpressionResult::RegisterOffset(reg_offset) => &self.span_to(reg_offset) - rhs,
            _ => Err(SpannedError::incorrect_type(
                vec!["number", "register", "register offset"],
                self,
            )),
        }
    }
}

impl Shl<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn shl(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) << rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Shr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn shr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) >> rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl Ashr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn ashr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => self.span_to(number).ashr(rhs),
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl BitAnd<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn bitand(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) & rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl BitOr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn bitor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) | rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}

impl BitXor<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, SpannedError>;
    fn bitxor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &self.span_to(number) ^ rhs,
            _ => Err(SpannedError::incorrect_type(vec!["number"], self)),
        }
    }
}
