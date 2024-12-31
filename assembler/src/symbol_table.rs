use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use internment::Intern;

use crate::{ast::Spanned, expression::expression_result::ExpressionResult, Error, Span};

pub struct SymbolAlreadyDefinedError(pub STEntry);

type Key = Intern<String>;
type Value = STEntry;

#[derive(Debug, Clone)]
pub struct STEntry {
    pub result: Spanned<ExpressionResult>,
    pub key_span: Span,
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

    pub fn insert(&mut self, key: Spanned<Key>, value: Spanned<ExpressionResult>) -> Option<Value> {
        self.table.insert(
            key.val,
            STEntry {
                result: value,
                key_span: key.span,
            },
        )
    }

    pub fn try_insert(
        &mut self,
        key: Spanned<Key>,
        value: Spanned<ExpressionResult>,
    ) -> Result<(), SymbolAlreadyDefinedError> {
        // need to call get and not just contains_key because error will contain the entry
        if let Some(entry) = self.get(&key.val) {
            return Err(SymbolAlreadyDefinedError(entry.clone()));
        }

        self.insert(key, value);
        Ok(())
    }
}

// some helper functions
pub fn get_identifier(
    ident: &Spanned<&Intern<String>>,
    symbol_table: &SymbolTable,
) -> Result<Value, Error> {
    if let Some(label_line) = symbol_table.get_recursive(ident.val) {
        Ok(label_line)
    } else {
        Err(Error::new(ident.span, "Missing identifier").with_label("Could not find identifier"))
    }
}
