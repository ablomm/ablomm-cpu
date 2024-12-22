use ariadne::Fmt;
use expression_result::{Ashr, AsmDeref, AsmRef, ExpressionResult, Number, Pos, String};

use crate::{
    ast::{Expression, Register, Spanned},
    symbol_table::{get_identifier, SymbolTable},
    Error, ATTENTION_COLOR,
};

pub mod expression_result;

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<Spanned<ExpressionResult>, Error> {
        match self.val {
            // there is a bunch of deref's here (i.e. **a) because a and b are a Box, which has
            // it's own as_ref() function, but we really need the Spanned::as_ref() function. No
            // deref's are needed if the Spanned::as_ref() method is named differently, but I
            // didn't like that
            Expression::Register(reg) => Ok(self.span_to(ExpressionResult::Register(*reg))),
            Expression::String(string) => {
                Ok(self.span_to(ExpressionResult::String(String(string.clone()))))
            }
            Expression::Number(a) => Ok(self.span_to(ExpressionResult::Number(Number(*a)))),
            Expression::Ident(a) => Ok(get_identifier(&self.span_to(a), symbol_table)?),
            Expression::Ref(expression) => {
                let a = (**expression).as_ref().eval(symbol_table)?;
                a.as_ref().asm_ref()
            }
            Expression::Deref(expression) => {
                let a = (**expression).as_ref().eval(symbol_table)?;
                a.as_ref().asm_deref()
            }
            Expression::Pos(a) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                a.as_ref().pos()
            }
            Expression::Neg(a) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                -&a.as_ref()
            }
            Expression::Not(a) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                !&a.as_ref()
            }
            Expression::Mul(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() * &b.as_ref()
            }
            Expression::Div(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() / &b.as_ref()
            }
            Expression::Remainder(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() % &b.as_ref()
            }
            Expression::Add(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() + &b.as_ref()
            }
            Expression::Sub(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() - &b.as_ref()
            }
            Expression::Shl(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() << &b.as_ref()
            }
            Expression::Shr(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() >> &b.as_ref()
            }
            Expression::Ashr(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                a.as_ref().ashr(&b.as_ref())
            }
            Expression::And(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() & &b.as_ref()
            }
            Expression::Or(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() | &b.as_ref()
            }
            Expression::Xor(a, b) => {
                let a = (**a).as_ref().eval(symbol_table)?;
                let b = (**b).as_ref().eval(symbol_table)?;
                &a.as_ref() ^ &b.as_ref()
            }
        }
    }
}
