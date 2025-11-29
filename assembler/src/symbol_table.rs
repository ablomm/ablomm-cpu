use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::{Rc, Weak},
};

use internment::Intern;

use crate::{
    Span, SpannedError,
    ast::Expression,
    expression::{LoopCheck, expression_result::ExpressionResult},
    span::Spanned,
};

pub type Key = Intern<String>;
pub type Value = STEntry;

pub mod setup;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub table: HashMap<Key, Value>,
    pub parent: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Debug, Clone)]
pub struct STEntry {
    pub symbol: Rc<RefCell<Symbol>>,

    // the span of the original definition identifier
    pub key_span: Span,

    // the span of the specifier that imports this symbol
    pub import_span: Option<Span>,

    // the span of the export statement of the import
    pub export_span: Option<Span>,
}

// not sure of a good name for this, but it's just the value that can be shared among multiple tables
#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: Spanned<SymbolValue>,
    // the symbol_table of where the identifier was defined, needed to evaluate imported identifiers
    // weak pointer because symbol_table already contains reference to the symbol
    pub symbol_table: Weak<RefCell<SymbolTable>>,
}

#[derive(Debug, Clone)]
pub enum SymbolValue {
    Result(ExpressionResult),
    Expression(Expression),
}

impl SymbolTable {
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: Eq + Hash + ?Sized,
        Key: std::borrow::Borrow<Q>,
    {
        self.table.contains_key(key)
    }

    pub fn contains_key_recursive<Q>(&self, key: &Q) -> bool
    where
        Q: Eq + Hash + ?Sized,
        Key: std::borrow::Borrow<Q>,
    {
        if self.contains_key(key) {
            return true;
        };

        if let Some(parent) = &self.parent {
            return parent.borrow().contains_key_recursive(key);
        }

        false
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Q: Eq + Hash + ?Sized,
        Key: std::borrow::Borrow<Q>,
    {
        self.table.get(key)
    }

    // owned value because parent may go out of scope while borrowed
    // I have attempted to return an enum of either &T or Ref<T>, but I have
    // gave up because of the borrow checker
    pub fn get_recursive<Q>(&self, key: &Q) -> Option<Value>
    where
        Q: Eq + Hash + ?Sized,
        Key: std::borrow::Borrow<Q>,
    {
        if let Some(entry) = self.get(key) {
            return Some(entry.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_recursive(key);
        }

        None
    }

    // just returns a nice error instead of Option
    pub fn try_get(&self, ident: &Spanned<&Key>) -> Result<Value, SpannedError> {
        self.get_recursive(ident.val).ok_or(
            SpannedError::new(ident.span, "Missing identifier")
                .with_label("Could not find identifier"),
        )
    }

    fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.table.insert(key, value)
    }

    fn try_insert(&mut self, key: Key, new_entry: Value) -> Result<(), SpannedError> {
        if let Some(entry) = self.table.get(&key) {
            return Err(SpannedError::identifier_already_defined(
                entry.key_span,
                entry.import_span,
                new_entry.key_span,
                new_entry.import_span,
            ));
        }
        self.insert(key, new_entry);

        Ok(())
    }
}

impl Symbol {
    pub fn try_get_result(
        &mut self,
        loop_check: &mut LoopCheck,
    ) -> Result<ExpressionResult, SpannedError> {
        match &self.value.val {
            // doesn't matter too much to clone since eval_with_loop_check() would need to clone anyways
            SymbolValue::Result(result) => Ok(result.clone()),
            SymbolValue::Expression(expression) => {
                let expression = self.value.span_to(expression);

                let symbol_table = self
                    .symbol_table
                    .upgrade()
                    // should never fail because all symbol tables are ultimately owned by the ast
                    // and all SymbolTables get dropped togther when Ast gets dropped (maybe use arena)
                    .expect("symbol's symbol table pointer invalid");

                match expression
                    .eval_with_loop_check(&symbol_table.borrow(), loop_check)
                    .map(|eval_return| eval_return.result)
                {
                    Ok(result) => {
                        self.value.val = SymbolValue::Result(result.clone());
                        Ok(result)
                    }
                    Err(error) => {
                        self.value.val = SymbolValue::Result(ExpressionResult::Error);
                        Err(error)
                    }
                }
            }
        }
    }
}
