use ariadne::Fmt;
use expression_result::ExpressionResult;

use crate::{
    ast::{Expression, Register, Spanned},
    symbol_table::{get_identifier, SymbolTable},
    Error, ATTENTION_COLOR,
};

pub mod expression_result;

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<Spanned<ExpressionResult>, Error> {
        match &self.val {
            // there is a bunch of deref's here (i.e. **a) because a and b are a Box, which has
            // it's own as_ref() function, but we really need the Spanned::as_ref() function. No
            // deref's are needed if the Spanned::as_ref() method is named differently, but I
            // didn't like that
            Expression::Register(reg) => {
                Ok(Spanned::new(ExpressionResult::Register(*reg), self.span))
            }
            Expression::String(string) => Ok(Spanned::new(
                ExpressionResult::String(string.clone()),
                self.span,
            )),
            Expression::Indirect(indirect) => {
                let indirect_inner = Spanned::new(&**indirect, self.span).eval(symbol_table)?;
                Ok(Spanned::new(
                    ExpressionResult::Indirect(Box::new(indirect_inner.val)),
                    self.span,
                ))
            }
            Expression::Number(a) => Ok(Spanned::new(ExpressionResult::Number(*a), self.span)),
            Expression::Ident(a) => Ok(get_identifier(&Spanned::new(a, self.span), symbol_table)?),
            Expression::Pos(a) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                a.as_ref().pos()
            }
            Expression::Neg(a) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                a.as_ref().neg()
            }
            Expression::Not(a) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                a.as_ref().not()
            }
            Expression::Mul(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().mul(&b.as_ref())
            }
            Expression::Div(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().div(&b.as_ref())
            }
            Expression::Remainder(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().remainder(&b.as_ref())
            }
            Expression::Add(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().add(&b.as_ref())
            }
            Expression::Sub(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().sub(&b.as_ref())
            }
            Expression::Shl(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().shl(&b.as_ref())
            }
            Expression::Shr(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().shr(&b.as_ref())
            }
            Expression::Ashr(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().ashr(&b.as_ref())
            }
            Expression::And(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().and(&b.as_ref())
            }
            Expression::Or(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().or(&b.as_ref())
            }
            Expression::Xor(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().xor(&b.as_ref())
            }
        }
    }
}
