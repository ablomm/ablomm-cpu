use super::*;

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&RegisterOffset> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &Spanned::new(number, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Add<&Spanned<&Number>> for &Spanned<&RegisterOffset> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Number>) -> Self::Output {
        let new_offset = (self.offset as u32).wrapping_add(**rhs.val) as i32;

        if new_offset == 0 {
            return Ok(ExpressionResult::Register(*self.reg));
        }

        Ok(ExpressionResult::RegisterOffset(RegisterOffset {
            reg: Spanned::new(*self.reg, self.reg.span),
            offset: new_offset,
        }))
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&RegisterOffset> {
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

impl Sub<&Spanned<&Number>> for &Spanned<&RegisterOffset> {
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&Number>) -> Self::Output {
        let new_offset = (self.offset as u32).wrapping_sub(**rhs.val) as i32;

        if new_offset == 0 {
            return Ok(ExpressionResult::Register(*self.reg));
        }

        Ok(ExpressionResult::RegisterOffset(RegisterOffset {
            reg: Spanned::new(*self.reg, self.reg.span),
            offset: new_offset,
        }))
    }
}
