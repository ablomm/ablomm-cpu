use crate::Span;
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

// just a struct to hold a span for error messages
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub val: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: Span) -> Self {
        Self { val, span }
    }
}

// just for simplicity (i.e. removes ".val" everywhere)
impl<T> Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        return &self.val;
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Block),
    Operation(Operation),
    Label(String),
    Assignment(Spanned<String>, Spanned<Expression>),
    Literal(Literal),
    Comment(String), // added because maybe it will be useful some day
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Spanned<Statement>>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub table: HashMap<String, i64>,
    pub parent: Option<Rc<RefCell<SymbolTable>>>,
}

impl SymbolTable {
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        return self.table.contains_key(key.as_ref());
    }

    pub fn contains_key_recursive(&self, key: &impl AsRef<str>) -> bool {
        let value = self.contains_key(key);
        if value {
            return true;
        };

        if let Some(parent) = &self.parent {
            return parent.borrow().contains_key_recursive(key);
        }

        return false;
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<i64> {
        return self.table.get(key.as_ref()).copied();
    }

    pub fn get_recursive(&self, key: &impl AsRef<str>) -> Option<i64> {
        let value = self.get(key);
        if let Some(value) = value {
            return Some(value);
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_recursive(key);
        }

        return None;
    }

    pub fn insert(&mut self, key: String, value: i64) -> Option<i64> {
        return self.table.insert(key, value);
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Expression(Expression),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Ident(String),
    Pos(Box<Spanned<Expression>>),
    Neg(Box<Spanned<Expression>>),
    Not(Box<Spanned<Expression>>),
    Mul(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Div(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Remainder(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Add(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Sub(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Shl(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Shr(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Ashr(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    And(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Or(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Xor(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub full_mnemonic: Spanned<FullMnemonic>,
    pub parameters: Spanned<Vec<Spanned<Parameter>>>,
}

#[derive(Debug, Clone)]
pub struct FullMnemonic {
    pub mnemonic: Spanned<Mnemonic>,
    pub modifiers: Spanned<Vec<Spanned<Modifier>>>,
}

#[derive(Debug, Copy, Clone)]
pub enum Mnemonic {
    NOP = 0,
    LD,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP,
    INT,
    // alu ops start with 0xf*
    PASS = 0xf0,
    AND,
    OR,
    XOR,
    NOT,
    ADD,
    ADDC,
    SUB,
    SUBB,
    NEG,
    SHL,
    SHR,
    ASHR,
}

#[derive(Debug, Copy, Clone)]
pub enum Modifier {
    Condition(Condition),
    AluModifier(AluModifier),
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    NONE = 0,
    EQ,
    NE,
    LTU,
    GTU,
    LEU,
    GEU,
    LTS,
    GTS,
    LES,
    GES,
}

#[derive(Debug, Copy, Clone)]
pub enum AluModifier {
    S,
    T,
}

#[derive(Debug, Copy, Clone)]
pub enum AluOpFlags {
    Immediate = 1 << 3,
    Reverse = 1 << 2,
    Loadn = 1 << 1,
    SetStatus = 1 << 0,
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Register(Register),
    RegisterOffset(Spanned<Register>, Spanned<Expression>),
    Expression(Expression),
    Indirect(Box<Parameter>),
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    FP,
    STATUS,
    SP,
    LR,
    PC,
}
