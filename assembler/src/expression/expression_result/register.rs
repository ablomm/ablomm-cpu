use std::ops::{Add, Sub};

use crate::{
    ast::Register,
    error::Error,
    expression::expression_result::{ExpressionResult, Number, RegisterOffset},
    span::Spanned,
};

impl Add<&Spanned<&ExpressionResult>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, Error>;

    fn add(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self + &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Add<&Spanned<&Option<Number>>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, Error>;

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
    type Output = Result<ExpressionResult, Error>;

    fn sub(self, rhs: &Spanned<&ExpressionResult>) -> Self::Output {
        match rhs.val {
            ExpressionResult::Number(number) => self - &rhs.span_to(number),
            _ => Err(Error::incorrect_type(vec!["number"], rhs)),
        }
    }
}

impl Sub<&Spanned<&Option<Number>>> for &Spanned<&Option<Register>> {
    type Output = Result<ExpressionResult, Error>;

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
