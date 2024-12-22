use ariadne::Fmt;
use expression_result::{Ashr, AsmDeref, AsmRef, ExpressionResult, Number, String};

use crate::{
    ast::{Expression, Register, Spanned},
    symbol_table::{get_identifier, SymbolTable},
    Error, ATTENTION_COLOR,
};

pub mod expression_result;

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<ExpressionResult, Error> {
        match self.val {
            // there is a bunch of deref's here (i.e. **a) because a and b are a Box, which has
            // it's own as_ref() function, but we really need the Spanned::as_ref() function. No
            // deref's are needed if the Spanned::as_ref() method is named differently, but I
            // didn't like that
            Expression::Register(reg) => Ok(ExpressionResult::Register(*reg)),
            Expression::String(string) => Ok(ExpressionResult::String(String(string.clone()))),
            Expression::Number(a) => Ok(ExpressionResult::Number(Number(*a))),
            Expression::Ident(a) => Ok(get_identifier(&self.span_to(a), symbol_table)?.val),
            Expression::Ref(a) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                a.as_ref().asm_ref()
            }
            Expression::Deref(a) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                a.as_ref().asm_deref()
            }
            Expression::Neg(a) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                -&a.as_ref()
            }
            Expression::Not(a) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                !&a.as_ref()
            }
            Expression::Mul(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() * &b.as_ref()
            }
            Expression::Div(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() / &b.as_ref()
            }
            Expression::Remainder(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() % &b.as_ref()
            }
            Expression::Add(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() + &b.as_ref()
            }
            Expression::Sub(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() - &b.as_ref()
            }
            Expression::Shl(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() << &b.as_ref()
            }
            Expression::Shr(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() >> &b.as_ref()
            }
            Expression::Ashr(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                a.as_ref().ashr(&b.as_ref())
            }
            Expression::And(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() & &b.as_ref()
            }
            Expression::Or(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() | &b.as_ref()
            }
            Expression::Xor(a, b) => {
                let a = a.span_to((**a).as_ref().eval(symbol_table)?);
                let b = a.span_to((**b).as_ref().eval(symbol_table)?);
                &a.as_ref() ^ &b.as_ref()
            }
        }
    }
}
