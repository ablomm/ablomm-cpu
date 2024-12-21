use super::*;

impl Pos for &Spanned<&RegisterOffset> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn pos(self) -> Self::Output {
        Ok(Spanned::new(
            ExpressionResult::RegisterOffset(*self.val),
            self.span,
        ))
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&RegisterOffset> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

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
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn add(self, rhs: &Spanned<&Number>) -> Self::Output {
        let new_offset = self.offset.wrapping_add(**rhs.val);

        if new_offset == 0 {
            return Ok(Spanned::new(
                ExpressionResult::Register(*self.reg),
                self.reg.span,
            ));
        }

        Ok(Spanned::new(
            ExpressionResult::RegisterOffset(RegisterOffset {
                reg: Spanned::new(*self.reg, self.reg.span),
                offset: new_offset,
            }),
            self.span.union(&rhs.span),
        ))
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&RegisterOffset> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

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
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn sub(self, rhs: &Spanned<&Number>) -> Self::Output {
        let new_offset = self.offset.wrapping_sub(**rhs.val);

        if new_offset == 0 {
            return Ok(Spanned::new(
                ExpressionResult::Register(*self.reg),
                self.reg.span,
            ));
        }

        Ok(Spanned::new(
            ExpressionResult::RegisterOffset(RegisterOffset {
                reg: Spanned::new(*self.reg, self.reg.span),
                offset: new_offset,
            }),
            self.span.union(&rhs.span),
        ))
    }
}
