use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub table: HashMap<String, u32>,
    pub parent: Option<Rc<RefCell<SymbolTable>>>,
}

impl SymbolTable {
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: Eq + Hash + ?Sized,
        String: std::borrow::Borrow<Q>,
    {
        return self.table.contains_key(key);
    }

    pub fn contains_key_recursive<Q>(&self, key: &Q) -> bool
    where
        Q: Eq + Hash + ?Sized,
        String: std::borrow::Borrow<Q>,
    {
        let value = self.contains_key(key);
        if value {
            return true;
        };

        if let Some(parent) = &self.parent {
            return parent.borrow().contains_key_recursive(key);
        }

        return false;
    }

    pub fn get<Q>(&self, key: &Q) -> Option<u32>
    where
        Q: Eq + Hash + ?Sized,
        String: std::borrow::Borrow<Q>,
    {
        return self.table.get(key).copied();
    }

    pub fn get_recursive<Q>(&self, key: &Q) -> Option<u32>
    where
        Q: Eq + Hash + ?Sized,
        String: std::borrow::Borrow<Q>,
    {
        let value = self.get(key);
        if let Some(value) = value {
            return Some(value);
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_recursive(key);
        }

        return None;
    }

    pub fn insert(&mut self, key: String, value: u32) -> Option<u32> {
        return self.table.insert(key, value);
    }
}
