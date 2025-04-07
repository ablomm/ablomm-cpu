use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use internment::Intern;

use crate::{expression::expression_result::ExpressionResult, span::Spanned, Span, SpannedError};

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

    // if it's allowable to import again
    pub updatable: bool,
}

impl STEntry {
    pub fn new(result: Spanned<ExpressionResult>, key_span: Span) -> Self {
        Self {
            result,
            key_span,
            import_span: None,
            export_span: None,
            updatable: false,
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

    pub fn updatable(mut self) -> Self {
        self.updatable = true;
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
        self.try_insert_expr(
            key,
            value.result,
            value.key_span,
            value.import_span,
            value.export_span,
            value.updatable,
        )
    }

    // try_insert calls this rather than the other way arround because this way ensures that
    // if the insert is skipped, then the expression will not be evaluated
    pub fn try_insert_expr<T: STInto<Spanned<ExpressionResult>>>(
        &mut self,
        key: Key,
        expression: T,
        key_span: Span,
        import_span: Option<Span>,
        export_span: Option<Span>,
        updatable: bool,
    ) -> Result<(), SpannedError> {
        if let Some(entry) = self.table.get_mut(&key) {
            if !entry.updatable {
                return Err(SpannedError::identifier_already_defined(
                    entry.key_span,
                    entry.import_span,
                    key_span,
                    import_span,
                ));
            } else if entry.result.is_known_val() {
                // no need to insert; value is already known
                entry.updatable = updatable;
                return Ok(());
            }
        }

        let result = expression.st_into(self)?;

        self.insert(
            key,
            STEntry {
                result,
                key_span,
                import_span,
                export_span,
                updatable,
            },
        );

        Ok(())
    }

    // flags every entry to be able to insert again. Useful for progressive passes containing more
    // and more info while keeping each pass only able to insert a symbol once
    pub fn set_entries_updatable(&mut self) {
        for entry in self.table.values_mut() {
            entry.updatable = true;
        }
    }
}

// trait similar to TryInto but can use the symbol_table
pub trait STInto<T> {
    fn st_into(self, symbol_table: &SymbolTable) -> Result<T, SpannedError>;
}

impl<T, U: Into<T>> STInto<T> for U {
    fn st_into(self, _symbol_table: &SymbolTable) -> Result<T, SpannedError> {
        Ok(self.into())
    }
}
