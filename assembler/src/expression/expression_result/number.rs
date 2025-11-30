use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use ariadne::Fmt;

use crate::{
    ast::Register,
    error::{ATTENTION_COLOR, Error, SpannedError},
    expression::expression_result::{Ashr, ExpressionResult, Number, RegisterOffset, String},
    span::Spanned,
};

impl Neg for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn neg(self) -> Self::Output {
        if let Some(val) = self.val {
            Ok(ExpressionResult::Number(Some(Number(
                -(**val as i32) as u32,
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Not for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn not(self) -> Self::Output {
        if let Some(val) = self.val {
            Ok(ExpressionResult::Number(Some(Number(!**val))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Mul<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn mul(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self * &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Mul<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn mul(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_mul(**rhs),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Div<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn div(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self / &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Div<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn div(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs_val)) = (self.val, rhs.val) {
            let rhs = rhs.span_to(rhs_val);
            if **rhs.val == 0 {
                return Err(Error::Spanned(Box::new(
                    SpannedError::new(rhs.span, "Division by 0").with_label(format!(
                        "Cannot divide by {}, and expression evaluates to {}",
                        "0".fg(ATTENTION_COLOR),
                        "0".fg(ATTENTION_COLOR)
                    )),
                )));
            }
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_div(**rhs.val),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Rem<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn rem(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self % &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Rem<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn rem(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs_val)) = (self.val, rhs.val) {
            let rhs = rhs.span_to(rhs_val);
            if **rhs.val == 0 {
                return Err(Error::Spanned(Box::new(
                    SpannedError::new(rhs.span, "Remainder by 0").with_label(format!(
                        "Cannot take remainder by {}, and expression evaluates to {}",
                        "0".fg(ATTENTION_COLOR),
                        "0".fg(ATTENTION_COLOR)
                    )),
                )));
            }
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_rem(**rhs.val),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &rhs.span_to(number),
            ExpressionResult::String(string) => self + &rhs.span_to(string),
            ExpressionResult::Register(register) => self + &rhs.span_to(register),
            ExpressionResult::RegisterOffset(reg_offset) => self + &rhs.span_to(reg_offset),
            _ => Err(Error::incorrect_type(
                vec!["number", "string", "register", "register offset"],
                rhs,
            )),
        }
    }
}

impl Add<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_add(**rhs),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Add<&Spanned<&Option<String>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<String>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::String(Some(String(
                lhs.to_string() + rhs,
            ))))
        } else {
            Ok(ExpressionResult::String(None))
        }
    }
}

impl Add<&Spanned<&Option<Register>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<Register>>) -> Self::Output {
        // delegate to register implementation (reg + num = num + reg)
        rhs + self
    }
}

impl Add<&Spanned<&Option<RegisterOffset>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<RegisterOffset>>) -> Self::Output {
        // delegate to register offset implementation (reg_offset + num = num + reg_offset)
        rhs + self
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self - &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Sub<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_sub(**rhs),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Shl<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn shl(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self << &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Shl<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn shl(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_shl(**rhs),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Shr<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn shr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self >> &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Shr<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn shr(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_shr(**rhs),
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl Ashr<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn ashr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self.ashr(&rhs.span_to(number)),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Ashr<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn ashr(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                (**lhs as i32).wrapping_shr(**rhs) as u32,
            ))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl BitAnd<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn bitand(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self & &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl BitAnd<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn bitand(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(**lhs & **rhs))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl BitOr<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn bitor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self | &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl BitOr<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn bitor(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(**lhs | **rhs))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}

impl BitXor<&Spanned<&ExpressionResult>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn bitxor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self ^ &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl BitXor<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn bitxor(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(**lhs ^ **rhs))))
        } else {
            Ok(ExpressionResult::Number(None))
        }
    }
}
