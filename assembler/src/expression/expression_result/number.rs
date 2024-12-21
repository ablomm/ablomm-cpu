use super::*;

impl Neg for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn neg(self) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            -(**self.val as i32) as u32,
        )))
    }
}

impl Not for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn not(self) -> Self::Output {
        Ok(ExpressionResult::Number(Number(!**self.val)))
    }
}

impl Mul<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn mul(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self * &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Mul<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn mul(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_mul(**rhs.val),
        )))
    }
}

impl Div<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn div(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self / &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Div<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn div(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_div(**rhs.val),
        )))
    }
}

impl Rem<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn rem(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self % &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Rem<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn rem(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_rem(**rhs.val),
        )))
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &Spanned::new(number, rhs.span),
            ExpressionResult::String(string) => self + &Spanned::new(string, rhs.span),
            ExpressionResult::Register(register) => self + &Spanned::new(register, rhs.span),
            ExpressionResult::RegisterOffset(reg_offset) => {
                self + &Spanned::new(reg_offset, rhs.span)
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

impl Add<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_add(**rhs.val),
        )))
    }
}

impl Add<&Spanned<&String>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&String>) -> Self::Output {
        Ok(ExpressionResult::String(String(self.to_string() + rhs.val)))
    }
}

impl Add<&Spanned<&Register>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Register>) -> Self::Output {
        // delegate to register implementation (reg + num = num + reg)
        rhs + self
    }
}

impl Add<&Spanned<&RegisterOffset>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&RegisterOffset>) -> Self::Output {
        // delegate to register offset implementation (reg_offset + num = num + reg_offset)
        rhs + self
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self - &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Sub<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_sub(**rhs.val),
        )))
    }
}

impl Shl<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn shl(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self << &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Shl<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn shl(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_shl(**rhs.val),
        )))
    }
}

impl Shr<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn shr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self >> &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Shr<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn shr(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            self.wrapping_shr(**rhs.val),
        )))
    }
}

impl Ashr<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn ashr(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self.ashr(&Spanned::new(number, rhs.span)),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Ashr<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn ashr(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(
            (**self.val as i32).wrapping_shr(**rhs.val) as u32,
        )))
    }
}

impl BitAnd<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn bitand(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self & &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl BitAnd<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn bitand(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(**self.val & **rhs.val)))
    }
}

impl BitOr<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn bitor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self | &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl BitOr<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn bitor(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(**self.val | **rhs.val)))
    }
}

impl BitXor<&Spanned<&ExpressionResult>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn bitxor(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self ^ &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl BitXor<&Spanned<&Number>> for &Spanned<&Number> {
    type Output = Result<ExpressionResult, Error>;

    fn bitxor(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::Number(Number(**self.val ^ **rhs.val)))
    }
}
