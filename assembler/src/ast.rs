use internment::Intern;

use crate::{span::Spanned, src::Src, symbol_table::SymbolTable};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct Ast {
    pub(crate) files: Vec<Spanned<File>>,
}

// this technically isn't needed, but I'm keeping it here just incase it becomes useful to
// distinguish between a normal block and a full file. you can get the Src through the Spanned<File>
#[derive(Debug, Clone)]
pub(crate) struct File {
    pub(crate) block: Block,
}

#[derive(Debug, Clone)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Spanned<Statement>>,
    pub(crate) symbol_table: Rc<RefCell<SymbolTable>>,
}

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    Block(Block),
    Operation(Operation),
    Label(Label),
    Assignment(Assignment),
    GenLiteral(Expression),
    Export(Vec<Spanned<Intern<String>>>),
    Import(Import),
    Error, // if statement has invalid syntax
}

#[derive(Debug, Clone)]
pub(crate) struct Label {
    pub(crate) export: bool,
    pub(crate) identifier: Spanned<Intern<String>>,
}

#[derive(Debug, Clone)]
pub(crate) struct Assignment {
    pub(crate) export: bool,
    pub(crate) identifier: Spanned<Intern<String>>,
    pub(crate) expression: Spanned<Expression>,
}

#[derive(Debug, Clone)]
pub(crate) struct Import {
    pub(crate) src: Spanned<Intern<Src>>,
    pub(crate) specifier: Spanned<ImportSpecifier>,
}

#[derive(Debug, Clone)]
pub(crate) enum ImportSpecifier {
    Named(Vec<Spanned<NamedImport>>), // import print as print2, thing from "lib/import.asm";
    Glob,                             // import * from "lib/import.asm";
}
#[derive(Debug, Clone)]
pub(crate) struct NamedImport {
    pub(crate) identifier: Spanned<Intern<String>>,
    pub(crate) alias: Option<Spanned<Intern<String>>>,
}

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Register(Register),
    String(String),
    Number(u32),
    Identifier(Intern<String>),
    Ref(Box<Spanned<Expression>>),
    Deref(Box<Spanned<Expression>>),
    #[allow(dead_code)]
    Neg(Box<Spanned<Expression>>), // not used, but may in future
    Not(Box<Spanned<Expression>>),
    Mul(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Div(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Rem(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
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
pub(crate) struct Operation {
    pub(crate) full_mnemonic: Spanned<FullMnemonic>,
    pub(crate) operands: Spanned<Vec<Spanned<Expression>>>,
}

#[derive(Debug, Clone)]
pub(crate) struct FullMnemonic {
    pub(crate) mnemonic: Spanned<AsmMnemonic>,
    pub(crate) modifiers: Spanned<Vec<Spanned<Modifier>>>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum AsmMnemonic {
    Nop,
    Ld,
    Push,
    Pop,
    Int,
    UnaryAlu(CpuMnemonic),
    BinaryAlu(CpuMnemonic),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum CpuMnemonic {
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
    Sub,
    Neg,
    Shl,
    Shr,
    Ashr,
    Rol,
    Ror,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Modifier {
    Condition(Condition),
    AluModifier(AluModifier),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Condition {
    #[allow(dead_code)]
    None = 0, // not used, but for completeness
    Eq,
    Ne,
    Neg,
    Pos,
    Vs,
    Vc,
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
pub(crate) enum AluModifier {
    S,
    T,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum AluOpFlags {
    Immediate = 1 << 3,
    Reverse = 1 << 2,
    Loadn = 1 << 1,
    SetStatus = 1 << 0,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Register {
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
    Status,
    Sp,
    Lr,
    Pclink,
    Pc,
}
