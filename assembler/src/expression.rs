use std::{cell::RefCell, collections::HashMap, rc::Rc};

use expression_result::{Ashr, AsmDeref, AsmRef, ExpressionResult, Number, String};
use indexmap::IndexMap;
use internment::Intern;

use crate::{
    Error, Span,
    ast::Expression,
    error::SpannedError,
    span::Spanned,
    symbol_table::{STEntry, Symbol, SymbolTable},
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

pub type LoopCheck = IndexMap<*const RefCell<Symbol>, (Span, Span)>;

pub struct EvalReturn {
    pub result: ExpressionResult,

    // the identifiers that are causing the result to be None. It is empty if the value is known
    // i.e. waiting for these identifiers to have known values before it itself can be known
    pub waiting_map: HashMap<Intern<std::string::String>, Span>,
}

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<EvalReturn, Error> {
        self.eval_with_loop_check(symbol_table, &mut IndexMap::new())
    }

    pub fn eval_with_loop_check(
        &self,
        symbol_table: &SymbolTable,
        loop_check: &mut LoopCheck,
    ) -> Result<EvalReturn, Error> {
        let mut waiting_map = HashMap::new();
        let result = match self.val {
            Expression::Register(register) => ExpressionResult::Register(Some(*register)),
            Expression::String(string) => ExpressionResult::String(Some(String(string.clone()))),
            Expression::Number(number) => ExpressionResult::Number(Some(Number(*number))),
            Expression::Identifier(identifier) => {
                let entry = symbol_table.try_get(&self.span_to(identifier))?;
                check_for_loops(&entry, self.span, loop_check)?;
                let result = entry.symbol.borrow_mut().try_get_result(loop_check)?;
                remove_from_loop_check(&entry, loop_check); // we got the result, so we can remove it
                result
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
    loop_check: &mut LoopCheck,
) -> Result<Spanned<ExpressionResult>, Error> {
    let EvalReturn {
        result,
        waiting_map: sub_waiting_map,
    } = (*val)
        .as_ref()
        .eval_with_loop_check(symbol_table, loop_check)?;
    waiting_map.extend(sub_waiting_map.iter());
    Ok(val.span_to(result))
}

fn check_for_loops(
    entry: &STEntry,
    identifier_span: Span,
    loop_check: &mut LoopCheck,
) -> Result<(), Error> {
    // just need a unique id for each symbol to detect loops
    let symbol_id = Rc::as_ptr(&entry.symbol);
    // the index at which the sequence starts to loop
    let loop_index = loop_check.get_index_of(&symbol_id);

    if let Some(loop_index) = loop_index {
        let mut error = SpannedError::new(identifier_span, "Circular definition");

        for (i, (key_span, identifier_span)) in loop_check.values().skip(loop_index).enumerate() {
            // first span is the key, then each subsequent span is the identifier within the
            // definition for a chain of dependencies
            let span = if i == 0 { key_span } else { identifier_span };
            error = error.with_label_span(*span, format!("Identifier {}", i + 1));
        }

        // identifier 1 is always the start of loop because we .skip(loop_index)
        error = error.with_label("This is identifier 1, causing a circular definition");

        Err(Error::Spanned(Box::new(error)))
    } else {
        loop_check.insert(symbol_id, (entry.key_span, identifier_span));
        Ok(())
    }
}

fn remove_from_loop_check(entry: &STEntry, loop_check: &mut LoopCheck) {
    let symbol_id = Rc::as_ptr(&entry.symbol);
    loop_check.shift_remove(&symbol_id);
}
