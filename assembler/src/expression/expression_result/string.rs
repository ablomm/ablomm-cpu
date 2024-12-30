use super::*;

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Option<String>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &rhs.span_to(number),
            ExpressionResult::String(string) => self + &rhs.span_to(string),
            _ => Err(Error::new(
                format!("Expected {}", "string".fg(ATTENTION_COLOR),),
                rhs.span,
            )),
        }
    }
}

impl Add<&Spanned<&Option<Number>>> for &Spanned<&Option<String>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<Number>>) -> Self::Output {
        if let (Some(lhs), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::String(Some(String(
                lhs.to_string() + &rhs.to_string(),
            ))))
        } else {
            Ok(ExpressionResult::String(None))
        }
    }
}

impl Add<&Spanned<&Option<String>>> for &Spanned<&Option<String>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&Option<String>>) -> Self::Output {
        if let (Some(val), Some(rhs)) = (self.val, rhs.val) {
            Ok(ExpressionResult::String(Some(String(
                val.to_string() + rhs,
            ))))
        } else {
            Ok(ExpressionResult::String(None))
        }
    }
}
