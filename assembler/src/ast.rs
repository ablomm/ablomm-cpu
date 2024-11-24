use crate::Span;
use std::ops::Deref;

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
    Operation(Operation),
    Label(String),
    Literal(Literal),
    Comment(String), // added because maybe it will be useful some day
}

#[derive(Debug, Clone)]
pub enum Literal {
    Expression(Expression),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(u32),
    Ident(String),
    Neg(Box<Spanned<Expression>>),
    Add(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Sub(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Mul(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Div(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
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
