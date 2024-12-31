use std::collections::HashMap;

use ariadne::Fmt;
use expression_result::{Ashr, AsmDeref, AsmRef, ExpressionResult, Number, String};
use internment::Intern;

use crate::{
    ast::{Expression, Register, Spanned},
    symbol_table::{get_identifier, SymbolTable},
    Error, Span, ATTENTION_COLOR,
};

pub mod expression_result;

pub struct EvalReturn {
    pub result: ExpressionResult,
    // the identifiers that are causing the result to be None. It is empty if the value is known
    // i.e. waiting for these identifiers to have known values before it itself can be known
    pub waiting_map: HashMap<Intern<std::string::String>, Span>,
}

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<EvalReturn, Error> {
        let mut waiting_map = HashMap::new();
        let result = match self.val {
            // there is a bunch of deref's here (i.e. **a) because a and b are a Box, which has
            // it's own as_ref() function, but we really need the Spanned::as_ref() function. No
            // deref's are needed if the Spanned::as_ref() method is named differently, but I
            // didn't like that
            Expression::Register(reg) => ExpressionResult::Register(Some(*reg)),
            Expression::String(string) => ExpressionResult::String(Some(String(string.clone()))),
            Expression::Number(a) => ExpressionResult::Number(Some(Number(*a))),
            Expression::Ident(a) => {
                let identifier = get_identifier(&self.span_to(a), symbol_table)?;
                match identifier.result.val {
                    ExpressionResult::Number(None)
                    | ExpressionResult::String(None)
                    | ExpressionResult::Register(None)
                    | ExpressionResult::RegisterOffset(None) => {
                        waiting_map.insert(*a, identifier.key_span);
                    }

                    _ => (),
                }
                get_identifier(&self.span_to(a), symbol_table)?.result.val
            }
            Expression::Ref(a) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                a.as_ref().asm_ref()?
            }
            Expression::Deref(a) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                a.as_ref().asm_deref()?
            }
            Expression::Neg(a) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (-&a.as_ref())?
            }
            Expression::Not(a) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (!&a.as_ref())?
            }
            Expression::Mul(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() * &b.as_ref())?
            }
            Expression::Div(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() / &b.as_ref())?
            }
            Expression::Remainder(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() % &b.as_ref())?
            }
            Expression::Add(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() + &b.as_ref())?
            }
            Expression::Sub(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() - &b.as_ref())?
            }
            Expression::Shl(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() << &b.as_ref())?
            }
            Expression::Shr(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() >> &b.as_ref())?
            }
            Expression::Ashr(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (a.as_ref().ashr(&b.as_ref()))?
            }
            Expression::And(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() & &b.as_ref())?
            }
            Expression::Or(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() | &b.as_ref())?
            }
            Expression::Xor(a, b) => {
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**a).as_ref().eval(symbol_table)?;
                let a = a.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                let EvalReturn {
                    result,
                    waiting_map: sub_waiting_map,
                } = (**b).as_ref().eval(symbol_table)?;
                let b = b.span_to(result);
                waiting_map.extend(sub_waiting_map.iter());
                (&a.as_ref() ^ &b.as_ref())?
            }
        };

        Ok(EvalReturn {
            result,
            waiting_map,
        })
    }
}
