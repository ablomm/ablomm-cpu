use chumsky::prelude::*;
use std::char;

#[derive(Debug, Clone)]
pub enum Register {
    R0,
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
    FP,
    STATUS,
    SP,
    PC,
}

#[derive(Debug, Clone)]
pub enum Mnemonic {
    And,
    Or,
    Xor,
    Not,
    Add,
    Addc,
    Sub,
    Subb,
    Shl,
    Shr,
    Ashr,
    Ld,
    St,
    Push,
    Pop,
    Int,
}

#[derive(Debug, Clone)]
pub enum Condition {
    Eq,
    Ne,
    Ltu,
    Gtu,
    Leu,
    Geu,
    Lts,
    Gts,
    Les,
    Ges,
}

#[derive(Debug, Clone)]
pub enum AluModifier {
    S,
    T,
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Condition(Condition),
    AluModifier(AluModifier),
}

#[derive(Debug)]
pub enum Parameter {
    Label(String),
    Number(u32),
    Register(Register),
}

#[derive(Debug)]
pub struct FullMnemonic {
    mnemonic: Box<Mnemonic>,
    modifiers: Vec<Modifier>,
}

#[derive(Debug)]
pub struct Operation {
    full_mnemonic: Box<FullMnemonic>,
    parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub enum Statement {
    Operation(Box<Operation>),
    Label(String),
}

pub fn parser() -> impl Parser<char, Vec<Statement>, Error = Simple<char>> {
    let label = text::ident();

    let hex_num = just("#0x")
        .ignore_then(text::int(16).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    let dec_num =
        just("#").ignore_then(text::int(10).map(|s: String| u32::from_str_radix(&s, 10).unwrap()));

    let number = hex_num.clone().or(dec_num.clone());

    let mnemonic = choice((
        text::keyword("and").to(Mnemonic::And),
        text::keyword("or").to(Mnemonic::Or),
        text::keyword("xor").to(Mnemonic::Xor),
        text::keyword("not").to(Mnemonic::Not),
        text::keyword("add").to(Mnemonic::Add),
        text::keyword("addc").to(Mnemonic::Addc),
        text::keyword("sub").to(Mnemonic::Sub),
        text::keyword("subb").to(Mnemonic::Subb),
        text::keyword("shl").to(Mnemonic::Shl),
        text::keyword("shr").to(Mnemonic::Shr),
        text::keyword("ashr").to(Mnemonic::Ashr),
        text::keyword("ld").to(Mnemonic::Ld),
        text::keyword("st").to(Mnemonic::St),
        text::keyword("push").to(Mnemonic::Push),
        text::keyword("pop").to(Mnemonic::Pop),
        text::keyword("int").to(Mnemonic::Int),
    ));

    let register = choice((
        text::keyword("r0").to(Register::R0),
        text::keyword("r1").to(Register::R1),
        text::keyword("r2").to(Register::R2),
        text::keyword("r3").to(Register::R3),
        text::keyword("r4").to(Register::R4),
        text::keyword("r5").to(Register::R5),
        text::keyword("r6").to(Register::R6),
        text::keyword("r7").to(Register::R7),
        text::keyword("r8").to(Register::R8),
        text::keyword("r9").to(Register::R9),
        text::keyword("r10").to(Register::R10),
        text::keyword("r11").to(Register::R11),
        text::keyword("fp").to(Register::FP),
        text::keyword("status").to(Register::STATUS),
        text::keyword("sp").to(Register::SP),
        text::keyword("pc").to(Register::PC),
    ));

    let alu_modifier = choice((
        text::keyword("s").to(AluModifier::S),
        text::keyword("T").to(AluModifier::T),
    ));

    let condition = choice((
        text::keyword("eq").to(Condition::Eq),
        text::keyword("ne").to(Condition::Ne),
        text::keyword("ltu").to(Condition::Ltu),
        text::keyword("gtu").to(Condition::Gtu),
        text::keyword("leu").to(Condition::Leu),
        text::keyword("geu").to(Condition::Geu),
        text::keyword("lts").to(Condition::Lts),
        text::keyword("gts").to(Condition::Gts),
        text::keyword("les").to(Condition::Les),
        text::keyword("ges").to(Condition::Ges),
    ));

    let modifier = just('.').ignore_then(
        alu_modifier
            .clone()
            .map(|am| Modifier::AluModifier(am))
            .or(condition.clone().map(|c| Modifier::Condition(c))),
    );

    let full_mnemonic =
        mnemonic
            .clone()
            .then(modifier.clone().repeated())
            .map(|(mnemonic, modifiers)| FullMnemonic {
                mnemonic: Box::new(mnemonic),
                modifiers,
            });

    let parameter = number
        .clone()
        .map(|n| Parameter::Number(n))
        .or(register.clone().map(|r| Parameter::Register(r)));

    let operation = full_mnemonic
        .clone()
        .padded()
        .then(parameter.clone().padded().separated_by(just(',')))
        .map(|(full_mnemonic, parameters)| Operation {
            full_mnemonic: Box::new(full_mnemonic),
            parameters,
        });

    let statement = operation
        .clone()
        .then_ignore(just(';'))
        .map(|o| Statement::Operation(Box::new(o)))
        .or(label
            .clone()
            .then_ignore(just(':'))
            .map(|l| Statement::Label(l)))
        .padded();

    return statement.repeated().then_ignore(end());
}
