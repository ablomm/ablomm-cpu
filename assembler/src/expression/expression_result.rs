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

#[derive(Debug, Clone)]
pub enum ExpressionResult {
    Number(Number),
    String(String),
    Register(Register),
    RegisterOffset(RegisterOffset),
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
            ExpressionResult::Indirect(indirect) => Spanned::new(indirect, self.span).asm_ref(),
            _ => Err(Error::new(
                format!("Expected {}", "indirect".fg(ATTENTION_COLOR)),
                self.span,
            )
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
            ExpressionResult::Number(number) => -&Spanned::new(number, self.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Not for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn not(self) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => !&Spanned::new(number, self.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Mul<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn mul(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) * rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Div<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn div(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) / rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Rem<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn rem(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) % rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) + rhs,
            ExpressionResult::String(string) => &Spanned::new(string, self.span) + rhs,
            ExpressionResult::Register(register) => &Spanned::new(register, self.span) + rhs,
            ExpressionResult::RegisterOffset(reg_offset) => {
                &Spanned::new(reg_offset, self.span) + rhs
            }
            _ => Err(Error::new(
                format!(
                    "Expected {}, {}, {}, or {}",
                    "number".fg(ATTENTION_COLOR),
                    "string".fg(ATTENTION_COLOR),
                    "register".fg(ATTENTION_COLOR),
                    "register offset".fg(ATTENTION_COLOR)
                ),
                rhs.span,
            )),
        }
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) - rhs,
            ExpressionResult::Register(register) => &Spanned::new(register, self.span) - rhs,
            ExpressionResult::RegisterOffset(reg_offset) => {
                &Spanned::new(reg_offset, self.span) - rhs
            }
            _ => Err(Error::new(
                format!(
                    "Expected {}, {}, or {}",
                    "number".fg(ATTENTION_COLOR),
                    "register".fg(ATTENTION_COLOR),
                    "register offset".fg(ATTENTION_COLOR)
                ),
                rhs.span,
            )),
        }
    }
}

impl Shl<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn shl(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) << rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Shr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn shr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) >> rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl Ashr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn ashr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => Spanned::new(number, self.span).ashr(rhs),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl BitAnd<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn bitand(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) & rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl BitOr<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn bitor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) | rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}

impl BitXor<&Spanned<&ExpressionResult>> for &Spanned<&ExpressionResult> {
    type Output = Result<ExpressionResult, Error>;
    fn bitxor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match self.val {
            ExpressionResult::Number(number) => &Spanned::new(number, self.span) ^ rhs,
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                self.span,
            )),
        }
    }
}
