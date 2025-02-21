use std::collections::HashMap;

use ariadne::Fmt;
use expression_result::{Ashr, AsmDeref, AsmRef, ExpressionResult, Number, String};
use internment::Intern;

use crate::{
    ast::{Expression, Register, Spanned},
    symbol_table::SymbolTable,
    Error, Span, ATTENTION_COLOR,
};

pub mod expression_result;

macro_rules! op {
    ($e:expr, $symbol_table:ident, $waiting_map:ident, $($val:ident),* ) => {{
        $(
            let $val = get_operand($val, $symbol_table, &mut $waiting_map)?;
            let $val = &$val.as_ref();
        )*
        $e
    }};
}

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
            Expression::Register(reg) => ExpressionResult::Register(Some(*reg)),
            Expression::String(string) => ExpressionResult::String(Some(String(string.clone()))),
            Expression::Number(a) => ExpressionResult::Number(Some(Number(*a))),
            Expression::Ident(a) => {
                let identifier = symbol_table.try_get(&self.span_to(a))?;
                match identifier.result.val {
                    ExpressionResult::Number(None)
                    | ExpressionResult::String(None)
                    | ExpressionResult::Register(None)
                    | ExpressionResult::RegisterOffset(None) => {
                        waiting_map.insert(*a, identifier.key_span);
                    }

                    _ => (),
                }
                identifier.result.val
            }
            Expression::Ref(a) => op!(a.asm_ref(), symbol_table, waiting_map, a)?,
            Expression::Deref(a) => op!(a.asm_deref(), symbol_table, waiting_map, a)?,
            Expression::Neg(a) => op!(-a, symbol_table, waiting_map, a)?,
            Expression::Not(a) => op!(!a, symbol_table, waiting_map, a)?,
            Expression::Mul(a, b) => op!(a * b, symbol_table, waiting_map, a, b)?,
            Expression::Div(a, b) => op!(a / b, symbol_table, waiting_map, a, b)?,
            Expression::Remainder(a, b) => op!(a % b, symbol_table, waiting_map, a, b)?,
            Expression::Add(a, b) => op!(a + b, symbol_table, waiting_map, a, b)?,
            Expression::Sub(a, b) => op!(a - b, symbol_table, waiting_map, a, b)?,
            Expression::Shl(a, b) => op!(a << b, symbol_table, waiting_map, a, b)?,
            Expression::Shr(a, b) => op!(a >> b, symbol_table, waiting_map, a, b)?,
            Expression::Ashr(a, b) => op!(a.ashr(b), symbol_table, waiting_map, a, b)?,
            Expression::And(a, b) => op!(a & b, symbol_table, waiting_map, a, b)?,
            Expression::Or(a, b) => op!(a | b, symbol_table, waiting_map, a, b)?,
            Expression::Xor(a, b) => op!(a ^ b, symbol_table, waiting_map, a, b)?,
        };

        Ok(EvalReturn {
            result,
            waiting_map,
        })
    }
}

pub fn get_operand(
    val: &Spanned<Expression>,
    symbol_table: &SymbolTable,
    waiting_map: &mut HashMap<Intern<std::string::String>, Span>,
) -> Result<Spanned<ExpressionResult>, Error> {
    let EvalReturn {
        result,
        waiting_map: sub_waiting_map,
    } = (*val).as_ref().eval(symbol_table)?;
    waiting_map.extend(sub_waiting_map.iter());
    Ok(val.span_to(result))
}
