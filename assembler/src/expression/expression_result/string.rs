use super::*;

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&String> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &Spanned::new(number, rhs.span),
            ExpressionResult::String(string) => self + &Spanned::new(string, rhs.span),
            _ => Err(Error::new(
                format!("Expected {}", "string".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Add<&Spanned<&Number>> for &Spanned<&String> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Number>) -> Self::Output {
        Ok(ExpressionResult::String(String(
            self.to_string() + &rhs.to_string(),
        )))
    }
}

impl Add<&Spanned<&String>> for &Spanned<&String> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&String>) -> Self::Output {
        Ok(ExpressionResult::String(String(self.to_string() + rhs)))
    }
}
