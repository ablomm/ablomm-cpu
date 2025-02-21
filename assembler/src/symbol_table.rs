use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use internment::Intern;

use crate::{ast::Spanned, expression::expression_result::ExpressionResult, Error, Span};

type Key = Intern<String>;
type Value = STEntry;

#[derive(Debug, Clone)]
pub struct STEntry {
    pub result: Spanned<ExpressionResult>,

    // the span of the original definition identifier
    pub key_span: Span,

    // the span of the specifier that imports this symbol
    pub import_span: Option<Span>,

    // the span of the export statement of the import
    pub export_span: Option<Span>,
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
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_recursive(key);
        }

        None
    }

    // just returns a nice error instead of Option
    pub fn try_get(&self, ident: &Spanned<&Key>) -> Result<Value, Error> {
        self.get_recursive(ident.val).ok_or(
            Error::new(ident.span, "Missing identifier").with_label("Could not find identifier"),
        )
    }

    pub fn insert(
        &mut self,
        key: Spanned<Key>,
        value: Spanned<ExpressionResult>,
        import_span: Option<Span>,
        export_span: Option<Span>,
    ) -> Option<Value> {
        self.table.insert(
            key.val,
            STEntry {
                result: value,
                key_span: key.span,
                import_span,
                export_span,
            },
        )
    }

    pub fn try_insert(
        &mut self,
        key: Spanned<Key>,
        value: Spanned<ExpressionResult>,
        import_span: Option<Span>,
        export_span: Option<Span>,
    ) -> Result<(), Error> {
        // need to call get and not just contains_key because error will contain the entry
        if let Some(entry) = self.get(&key.val) {
            return Err(Error::identifier_already_defined(
                entry.import_span.unwrap_or(entry.key_span),
                key.span,
            ));
        }

        self.insert(key, value, import_span, export_span);
        Ok(())
    }
}
