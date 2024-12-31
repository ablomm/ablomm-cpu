use super::*;

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Option<RegisterOffset>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &rhs.span_to(number),
            _ => Err(Error::new(rhs.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                rhs.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Add<&Spanned<&Option<Number>>> for &Spanned<&Option<RegisterOffset>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            let new_offset = (lhs.offset as u32).wrapping_add(**rhs) as i32;

            Ok(ExpressionResult::RegisterOffset(Some(RegisterOffset {
                reg: Spanned::new(*lhs.reg, lhs.reg.span),
                offset: new_offset,
            })))
        } else {
            Ok(ExpressionResult::RegisterOffset(None))
        }
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&Option<RegisterOffset>> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self - &rhs.span_to(number),
            _ => Err(Error::new(rhs.span, "Incorrect type").with_label(format!(
                "Expected {}, but found {}",
                "number".fg(ATTENTION_COLOR),
                rhs.fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Sub<&Spanned<&Option<Number>>> for &Spanned<&Option<RegisterOffset>> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            let new_offset = (lhs.offset as u32).wrapping_sub(**rhs) as i32;

            Ok(ExpressionResult::RegisterOffset(Some(RegisterOffset {
                reg: Spanned::new(*lhs.reg, lhs.reg.span),
                offset: new_offset,
            })))
        } else {
            Ok(ExpressionResult::RegisterOffset(None))
        }
    }
}
