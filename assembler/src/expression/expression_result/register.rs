use super::*;

impl Pos for &Spanned<&Register> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn pos(self) -> Self::Output {
        Ok(Spanned::new(
            ExpressionResult::Register(*self.val),
            self.span,
        ))
    }
}

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Register> {
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

impl Add<&Spanned<&Number>> for &Spanned<&Register> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn add(self, rhs: &Spanned<&Number>) -> Self::Output {
        let reg_offset = RegisterOffset {
            reg: Spanned::new(*self.val, self.span),
            offset: 0,
        };

        // delegate to register offset implmentation (reg = reg + 0)
        &Spanned::new(&reg_offset, self.span) + rhs
    }
}

impl Sub<&Spanned<&ExpressionResult>> for &Spanned<&Register> {
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

impl Sub<&Spanned<&Number>> for &Spanned<&Register> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn sub(self, rhs: &Spanned<&Number>) -> Self::Output {
        let reg_offset = RegisterOffset {
            reg: Spanned::new(*self.val, self.span),
            offset: 0,
        };

        // delegate to register offset implmentation (reg = reg + 0)
        &Spanned::new(&reg_offset, self.span) - rhs
    }
}
