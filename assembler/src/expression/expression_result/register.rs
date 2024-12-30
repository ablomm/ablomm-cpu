use super::*;

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &rhs.span_to(number),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Add<&Spanned<&Option<Number>>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let Some(val) = self.val {
            let reg_offset = RegisterOffset {
                reg: Spanned::new(*val, self.span),
                offset: 0,
            };

            // delegate to register offset implmentation (reg = reg + 0)
            &self.span_to(&Some(reg_offset)) + rhs
        } else {
            Ok(ExpressionResult::RegisterOffset(None))
        }
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&Option<Register>> {
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

impl Sub<&Spanned<&Option<Number>>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let Some(val) = self.val {
            let reg_offset = RegisterOffset {
                reg: Spanned::new(*val, self.span),
                offset: 0,
            };

            // delegate to register offset implmentation (reg = reg + 0)
            &self.span_to(&Some(reg_offset)) - rhs
        } else {
            Ok(ExpressionResult::RegisterOffset(None))
        }
    }
}
