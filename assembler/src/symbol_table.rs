use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use internment::Intern;

use crate::{
    ast::{Expression, Spanned},
    expression::expression_result::ExpressionResult,
    Span, SpannedError,
};

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

    // final means not allowable to import again
    pub r#final: bool,
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
    pub fn try_get(&self, ident: &Spanned<&Key>) -> Result<Value, SpannedError> {
        self.get_recursive(ident.val).ok_or(
            SpannedError::new(ident.span, "Missing identifier")
                .with_label("Could not find identifier"),
        )
    }

    fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.table.insert(key, value)
    }

    pub fn try_insert(&mut self, key: Key, value: Value) -> Result<(), SpannedError> {
        if let Some(entry) = self.get(&key) {
            if entry.r#final {
                return Err(SpannedError::identifier_already_defined(
                    entry.key_span,
                    entry.import_span,
                    value.key_span,
                    value.import_span,
                ));
            }
        }

        self.insert(key, value);
        Ok(())
    }

    pub fn try_insert_expr(
        &mut self,
        key: Key,
        expression: &Spanned<Expression>,
        key_span: Span,
        import_span: Option<Span>,
        export_span: Option<Span>,
        r#final: bool,
    ) -> Result<(), SpannedError> {
        let result = expression.span_to(expression.as_ref().eval(self)?.result);

        self.try_insert(
            key,
            STEntry {
                result,
                key_span,
                import_span,
                export_span,
                r#final,
            },
        )
    }
}
