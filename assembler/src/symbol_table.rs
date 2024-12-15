use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use internment::Intern;

use crate::{ast::Spanned, expression::expression_result::ExpressionResult, Error};

pub struct SymbolError;

type Key = Intern<String>;
type Value = Spanned<ExpressionResult>;

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
        let value = self.contains_key(key);
        if value {
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
        let value = self.get(key);
        if let Some(value) = value {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_recursive(key);
        }

        None
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.table.insert(key, value)
    }

    pub fn try_insert(&mut self, key: Key, value: Value) -> Result<(), SymbolError> {
        // when map_try_insert is released, we can just use that, for it's my own implementaiton
        if self.table.contains_key(&key) {
            return Err(SymbolError);
        }

        self.table.insert(key, value);
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
        Err(Error::new("Could not find identifier", ident.span))
    }
}
