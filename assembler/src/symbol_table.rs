use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use internment::Intern;

use crate::{
    Span, SpannedError, ast::Expression, expression::expression_result::ExpressionResult,
    span::Spanned,
};

pub type Key = Intern<String>;
pub type Value = Rc<RefCell<STEntry>>;

pub mod setup;

#[derive(Debug, Clone)]
pub struct STEntry {
    pub expression: Option<Spanned<Expression>>,
    pub result: Option<Spanned<ExpressionResult>>,

    // the span of the original definition identifier
    pub key_span: Span,

    // the span of the specifier that imports this symbol
    pub import_span: Option<Span>,

    // the span of the export statement of the import
    pub export_span: Option<Span>,
}

impl STEntry {
    pub fn new(key_span: Span) -> Self {
        Self {
            expression: None,
            result: None,
            key_span,
            import_span: None,
            export_span: None,
        }
    }

    pub fn with_expression(mut self, expression: Spanned<Expression>) -> Self {
        self.expression = Some(expression);
        self
    }

    pub fn with_result(mut self, result: Spanned<ExpressionResult>) -> Self {
        self.result = Some(result);
        self
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
        if let Some(value) = self.get(key) {
            return Some(Rc::clone(value));
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
    pub fn try_get_with_result(&self, ident: &Spanned<&Key>) -> Result<Value, SpannedError> {
        let binding = self.try_get(ident)?;
        let mut entry = binding.borrow_mut();

        if entry.result.is_none() {
            let expression = entry
                .expression
                .as_ref()
                .expect("Symbol has neither expression nor result");
            let expression_result = expression.as_ref().eval(&self);
            (*entry).result = Some(expression.span_to(expression_result?.result));
        }

        Ok(binding.clone())
    }

    fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.table.insert(key, value)
    }

    fn try_insert(&mut self, key: Key, value: Value) -> Result<(), SpannedError> {
        if let Some(entry) = self.table.get(&key) {
            let entry = entry.borrow();
            let value = value.borrow();
            return Err(SpannedError::identifier_already_defined(
                entry.key_span,
                entry.import_span,
                value.key_span,
                value.import_span,
            ));
        }
        self.insert(key, value);

        Ok(())
    }
}
