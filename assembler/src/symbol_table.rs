use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use indexmap::IndexMap;
use internment::Intern;

use crate::{
    Span, SpannedError, ast::Expression, expression::expression_result::ExpressionResult,
    span::Spanned,
};

pub type Key = Intern<String>;
pub type Value = STEntry;

pub mod setup;

#[derive(Debug, Clone)]
pub struct STEntry {
    pub symbol: Rc<RefCell<Symbol>>,

    //TODO: Not have these shared (in the Rc)
    // the span of the original definition identifier
    pub key_span: Span,

    // the span of the specifier that imports this symbol
    pub import_span: Option<Span>,

    // the span of the export statement of the import
    pub export_span: Option<Span>,
}

impl STEntry {
    pub fn new(symbol: Rc<RefCell<Symbol>>, key_span: Span) -> Self {
        Self {
            symbol,
            key_span,
            import_span: None,
            export_span: None,
        }
    }

    pub fn with_import_span(mut self, import_span: Span) -> Self {
        self.import_span = Some(import_span);
        self
    }

    pub fn with_export_span(mut self, export_span: Span) -> Self {
        self.export_span = Some(export_span);
        self
    }
}

// not sure of a good name for this, but it's just the value that can be shared among multiple
// tables
#[derive(Debug, Clone)]
pub struct Symbol {
    pub result: Option<Spanned<ExpressionResult>>,
    pub expression: Option<Spanned<Expression>>,

    // the symbol_table of where the identifier was defined, needed to evaluate imported
    // identifiers
    pub symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Symbol {
    pub fn new(symbol_table: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            result: None,
            expression: None,
            symbol_table,
        }
    }

    pub fn with_result(mut self, result: Spanned<ExpressionResult>) -> Self {
        self.result = Some(result);
        self
    }

    pub fn with_expression(mut self, expression: Spanned<Expression>) -> Self {
        self.expression = Some(expression);
        self
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub table: HashMap<Key, Value>,
    pub parent: Option<Rc<RefCell<SymbolTable>>>,
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

    // gets Value with the result field set (evaluates expresssion)
    pub fn try_get_with_result(
        &self,
        ident: &Spanned<&Key>,
        // using pointer as a unique id for map to determine equality
        loop_check: &mut IndexMap<*const RefCell<Symbol>, Span>,
    ) -> Result<Value, SpannedError> {
        let entry = self.try_get(ident)?;
        let symbol_id = Rc::as_ptr(&entry.symbol);

        if loop_check.contains_key(&symbol_id) {
            let mut error = SpannedError::new(entry.key_span, "Circular definition");

            for (i, (_, back_span)) in loop_check.iter().enumerate() {
                error =
                    error.with_label_span(*back_span, format!("Assignment {} of the loop", i + 1));
            }

            error = error.with_label("This completes the loop, causing a circular definiton");

            return Err(error);
        }
        loop_check.insert(symbol_id, entry.key_span);

        let mut symbol = entry.symbol.borrow_mut();

        if symbol.result.is_none() {
            let expression = symbol
                .expression
                .as_ref()
                .expect("Symbol has neither expression nor result");
            let expression_result = expression
                .as_ref()
                .eval_with_loop_check(&symbol.symbol_table.borrow(), loop_check);
            symbol.result = Some(expression.span_to(expression_result?.result));
        }

        loop_check.shift_remove(&symbol_id);

        Ok(entry.clone())
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
