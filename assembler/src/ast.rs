use internment::Intern;

use crate::{src::Src, symbol_table::SymbolTable, Span};
use std::{cell::RefCell, ops::Deref, rc::Rc};

// just a struct to hold a span for error messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub val: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: Span) -> Self {
        Self { val, span }
    }

    // converts &Spanned<T> to Spanned<&T>
    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned::new(&self.val, self.span)
    }

    pub fn span_to<V>(&self, to: V) -> Spanned<V> {
        Spanned::new(to, self.span)
    }
}

// just for simplicity (i.e. removes ".val" everywhere)
impl<T> Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub files: Vec<Spanned<File>>,
}

#[derive(Debug, Clone)]
pub struct File {
    pub src: Intern<Src>,
    pub start_address: u32,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Spanned<Statement>>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Block),
    Operation(Operation),
    Label(Label),
    Assignment(Assignment),
    Literal(Expression),
    Export(Vec<Spanned<Intern<String>>>),
    Import(Import),
    #[allow(dead_code)]
    Comment(String), // added because maybe it will be useful some day; not used
}

#[derive(Debug, Clone)]
pub struct Label {
    pub export: bool,
    pub identifier: Spanned<Intern<String>>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub export: bool,
    pub identifier: Spanned<Intern<String>>,
    pub expression: Spanned<Expression>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub file: Spanned<String>,
    pub specifier: Spanned<ImportSpecifier>,
}

#[derive(Debug, Clone)]
pub enum ImportSpecifier {
    Named(Vec<Spanned<NamedImport>>), // import print as print2, thing from "lib/import.asm";
    Blob,                             // import * from "lib/import.asm";
}
#[derive(Debug, Clone)]
pub struct NamedImport {
    pub identifier: Spanned<Intern<String>>,
    pub alias: Option<Spanned<Intern<String>>>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Register(Register),
    String(String),
    Number(u32),
    Ident(Intern<String>),
    Pos(Box<Spanned<Expression>>),
    Ref(Box<Spanned<Expression>>),
    Deref(Box<Spanned<Expression>>),
    #[allow(dead_code)]
    Neg(Box<Spanned<Expression>>), // not used, but may in future
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
    pub operands: Spanned<Vec<Spanned<Expression>>>,
}

#[derive(Debug, Clone)]
pub struct FullMnemonic {
    pub mnemonic: Spanned<Mnemonic>,
    pub modifiers: Spanned<Vec<Spanned<Modifier>>>,
}

#[derive(Debug, Copy, Clone)]
pub enum Mnemonic {
    Nop = 0,
    Ld,
    Ldr,
    Ldi,
    St,
    Str,
    Push,
    Pop,
    Int,
    // alu ops start with 0xf*
    Pass = 0xf0,
    And,
    Or,
    Xor,
    Not,
    Add,
    Addc,
    Sub,
    Subb,
    Neg,
    Shl,
    Shr,
    Ashr,
}

#[derive(Debug, Copy, Clone)]
pub enum Modifier {
    Condition(Condition),
    AluModifier(AluModifier),
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    #[allow(dead_code)]
    None = 0, // not used, but for completeness
    Eq,
    Ne,
    Ult,
    Ugt,
    Ule,
    Uge,
    Slt,
    Sgt,
    Sle,
    Sge,
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
    R11,
    Status,
    Sp,
    Lr,
    Pc,
}
