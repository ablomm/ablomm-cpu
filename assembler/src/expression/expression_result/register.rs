use super::*;

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, SpannedError>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &rhs.span_to(number),
            _ => Err(SpannedError::incorrect_value(
                rhs.span,
                "type",
                vec!["number"],
                Some(rhs.val),
            )),
        }
    }
}

impl Add<&Spanned<&Option<Number>>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, SpannedError>;

    fn add(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let Some(lhs) = self.val {
            let reg_offset = RegisterOffset {
                reg: Spanned::new(*lhs, self.span),
                offset: 0,
            };

            // delegate to register offset implementation (reg = reg + 0)
            &self.span_to(&Some(reg_offset)) + rhs
        } else {
            Ok(ExpressionResult::RegisterOffset(None))
        }
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, SpannedError>;

    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self - &rhs.span_to(number),
            _ => Err(SpannedError::incorrect_value(
                rhs.span,
                "type",
                vec!["number"],
                Some(rhs.val),
            )),
        }
    }
}

impl Sub<&Spanned<&Option<Number>>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, SpannedError>;

    fn sub(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let Some(lhs) = self.val {
            let reg_offset = RegisterOffset {
                reg: Spanned::new(*lhs, self.span),
                offset: 0,
            };

            // delegate to register offset implementation (reg = reg + 0)
            &self.span_to(&Some(reg_offset)) - rhs
        } else {
            Ok(ExpressionResult::RegisterOffset(None))
        }
    }
}
