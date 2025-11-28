use std::{cell::RefCell, collections::HashMap};

use ariadne::Fmt;
use expression_result::{Ashr, AsmDeref, AsmRef, ExpressionResult, Number, String};
use indexmap::IndexMap;
use internment::Intern;

use crate::{
    ATTENTION_COLOR, Span, SpannedError,
    ast::{Expression, Register},
    span::Spanned,
    symbol_table::{Symbol, SymbolTable},
};

pub mod expression_result;

macro_rules! op {
    ($e:expr, $symbol_table:ident, $waiting_map:ident, $loop_check:ident, $($val:ident),* ) => {{
        $(
            let $val = get_operand($val, $symbol_table, &mut $waiting_map, $loop_check)?;
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
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<EvalReturn, SpannedError> {
        self.eval_with_loop_check(symbol_table, &mut IndexMap::new())
    }

    pub fn eval_with_loop_check(
        &self,
        symbol_table: &SymbolTable,
        loop_check: &mut IndexMap<*const RefCell<Symbol>, Span>,
    ) -> Result<EvalReturn, SpannedError> {
        let mut waiting_map = HashMap::new();
        let result = match self.val {
            Expression::Register(register) => ExpressionResult::Register(Some(*register)),
            Expression::String(string) => ExpressionResult::String(Some(String(string.clone()))),
            Expression::Number(number) => ExpressionResult::Number(Some(Number(*number))),
            Expression::Identifier(identifier) => {
                let entry =
                    symbol_table.try_get_with_result(&self.span_to(identifier), loop_check)?;
                let symbol = entry.symbol.borrow();
                let result = symbol.result.clone().unwrap_or_else(|| {
                    panic!(
                        "Identifier '{}' does not contain result after get_with_result",
                        identifier
                    )
                });
                if !result.val.is_known_val() {
                    waiting_map.insert(*identifier, entry.key_span);
                }
                result.val
            }
            Expression::Ref(a) => op!(a.asm_ref(), symbol_table, waiting_map, loop_check, a)?,
            Expression::Deref(a) => op!(a.asm_deref(), symbol_table, waiting_map, loop_check, a)?,
            Expression::Neg(a) => op!(-a, symbol_table, waiting_map, loop_check, a)?,
            Expression::Not(a) => op!(!a, symbol_table, waiting_map, loop_check, a)?,
            Expression::Mul(a, b) => op!(a * b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Div(a, b) => op!(a / b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Rem(a, b) => op!(a % b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Add(a, b) => op!(a + b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Sub(a, b) => op!(a - b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Shl(a, b) => op!(a << b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Shr(a, b) => op!(a >> b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Ashr(a, b) => op!(a.ashr(b), symbol_table, waiting_map, loop_check, a, b)?,
            Expression::And(a, b) => op!(a & b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Or(a, b) => op!(a | b, symbol_table, waiting_map, loop_check, a, b)?,
            Expression::Xor(a, b) => op!(a ^ b, symbol_table, waiting_map, loop_check, a, b)?,
        };

        Ok(EvalReturn {
            result,
            waiting_map,
        })
    }
}

fn get_operand(
    val: &Spanned<Expression>,
    symbol_table: &SymbolTable,
    waiting_map: &mut HashMap<Intern<std::string::String>, Span>,
    loop_check: &mut IndexMap<*const RefCell<Symbol>, Span>,
) -> Result<Spanned<ExpressionResult>, SpannedError> {
    let EvalReturn {
        result,
        waiting_map: sub_waiting_map,
    } = (*val)
        .as_ref()
        .eval_with_loop_check(symbol_table, loop_check)?;
    waiting_map.extend(sub_waiting_map.iter());
    Ok(val.span_to(result))
}
