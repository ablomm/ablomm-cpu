use super::*;

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
            Ok(ExpressionResult::Number(Some(Number(
                !(**val as i32) as u32,
            ))))
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Div<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn div(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_div(**rhs),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Rem<&Spanned<&Option<Number>>> for &Spanned<&Option<Number>> {
    type Output = Result<ExpressionResult, Error>;

    fn rem(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::Number(Some(Number(
                lhs.wrapping_rem(**rhs),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
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
