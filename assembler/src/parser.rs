use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use chumsky::{prelude::*, primitive::Custom};
use std::char;

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
    FP,
    STATUS,
    SP,
    PC,
}

impl Register {
    pub fn generate(&self) -> u32 {
        return *self as u32;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Mnemonic {
    // PASSA = 0, PASSB = 1
    AND = 2,
    OR,
    XOR,
    NOT,
    ADD,
    ADDC,
    SUB,
    SUBB,
    SHL,
    SHR,
    ASHR,
    LD, // instruction depends on operands 0x10 to 0x12
    ST, // instruction depends on operands 0x13 to 0x15
    PUSH = 0x16,
    POP,
    INT,
}

impl Mnemonic {
    pub fn generate(&self) -> u32 {
        return *self as u32;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    // 0 is NONE, but no need for the assembler
    EQ = 1,
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

impl Condition {
    pub fn generate(&self) -> u32 {
        return *self as u32;
    }
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
    Indirect(Box<Parameter>),
}

#[derive(Debug)]
pub struct FullMnemonic {
    pub mnemonic: Mnemonic,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug)]
pub struct Operation {
    pub full_mnemonic: FullMnemonic,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub enum Statement {
    Operation(Operation),
    Label(String),
}

pub fn parser() -> impl Parser<char, Vec<Statement>, Error = Simple<char>> {
    let label = text::ident();

    let bin_num =
        just("0b").ignore_then(text::int(2).map(|s: String| u32::from_str_radix(&s, 2).unwrap()));
    let oct_num =
        just("0o").ignore_then(text::int(8).map(|s: String| u32::from_str_radix(&s, 8).unwrap()));
    let hex_num =
        just("0x").ignore_then(text::int(16).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    let dec_num = text::int(10).map(|s: String| u32::from_str_radix(&s, 10).unwrap());

    let number = choice((bin_num, oct_num, hex_num, dec_num));

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

    let parameter = recursive(|parameter| {
        let indirect = parameter.delimited_by(just('['), just(']'));
        return choice((
            number.map(Parameter::Number),
            register.map(Parameter::Register),
            label.map(Parameter::Label),
            indirect.map(|i| Parameter::Indirect(Box::new(i))),
        ));
    });

    let alu_modifier = choice((
        text::keyword("s").to(AluModifier::S),
        text::keyword("t").to(AluModifier::T),
    ));

    let condition = choice((
        text::keyword("eq").to(Condition::EQ),
        text::keyword("ne").to(Condition::NE),
        text::keyword("ltu").to(Condition::LTU),
        text::keyword("gtu").to(Condition::GTU),
        text::keyword("leu").to(Condition::LEU),
        text::keyword("geu").to(Condition::GEU),
        text::keyword("lts").to(Condition::LTS),
        text::keyword("gts").to(Condition::GTS),
        text::keyword("les").to(Condition::LES),
        text::keyword("ges").to(Condition::GES),
    ));

    let modifier = just('.').ignore_then(choice((
        alu_modifier.map(Modifier::AluModifier),
        condition.map(Modifier::Condition),
    )));

    let mnemonic = choice((
        text::keyword("and").to(Mnemonic::AND),
        text::keyword("or").to(Mnemonic::OR),
        text::keyword("xor").to(Mnemonic::XOR),
        text::keyword("not").to(Mnemonic::NOT),
        text::keyword("add").to(Mnemonic::ADD),
        text::keyword("addc").to(Mnemonic::ADDC),
        text::keyword("sub").to(Mnemonic::SUB),
        text::keyword("subb").to(Mnemonic::SUBB),
        text::keyword("shl").to(Mnemonic::SHL),
        text::keyword("shr").to(Mnemonic::SHR),
        text::keyword("ashr").to(Mnemonic::ASHR),
        text::keyword("ld").to(Mnemonic::LD),
        text::keyword("st").to(Mnemonic::ST),
        text::keyword("push").to(Mnemonic::PUSH),
        text::keyword("pop").to(Mnemonic::POP),
        text::keyword("int").to(Mnemonic::INT),
    ));

    let full_mnemonic = mnemonic
        .then(modifier.repeated())
        .map(|(mnemonic, modifiers)| FullMnemonic {
            mnemonic,
            modifiers,
        });

    let operation = full_mnemonic
        .padded()
        .then(parameter.padded().separated_by(just(',')))
        .map(|(full_mnemonic, parameters)| Operation {
            full_mnemonic,
            parameters,
        });

    let statement = choice((
        operation.then_ignore(just(';')).map(Statement::Operation),
        label.then_ignore(just(':')).map(Statement::Label),
    ))
    .padded();

    return statement.repeated().then_ignore(end());
}

pub fn print_syntax_error(errors: &Vec<Simple<char>>) {
    Report::build(ReportKind::Error, "program.asm", 12)
        .with_code(1)
        .with_message("Parse error")
        .with_labels(errors.iter().map(error_label).flatten())
        .finish()
        .print(("program.asm", Source::from(include_str!("../program.asm"))))
        .unwrap();
}

fn error_label(error: &Simple<char>) -> Vec<Label<(&str, std::ops::Range<usize>)>> {
    match error.reason() {
        chumsky::error::SimpleReason::Unexpected => {
            return vec![Label::new(("program.asm", error.span())).with_message(error_msg(error))]
        }
        chumsky::error::SimpleReason::Unclosed { span, delimiter } => {
            return vec![
                Label::new(("program.asm", error.span())).with_message(error_msg(error)),
                Label::new(("program.asm", span.clone()))
                    .with_message(format!("unclosed {delimiter}")),
            ]
        }
        _ => return vec![Label::new(("program.asm", error.span())).with_message(error_msg(error))],
    }
}

fn error_msg(error: &Simple<char>) -> String {
    match error.reason() {
        chumsky::error::SimpleReason::Unexpected => {
            // all of this just to get expected characters...
            let mut expected_values = Vec::new();
            for exp in error.expected() {
                if let Some(value) = exp {
                    expected_values.push(format!("'{}'", value.to_string()));
                }
            }

            let mut found_value = String::new();
            if let Some(value) = error.found() {
                found_value = value.to_string();
            }

            return format!(
                "Expected: {} but Found: '{}'",
                expected_values.join(" or "),
                found_value
            );
        }
        chumsky::error::SimpleReason::Unclosed { span, delimiter } => return "test2".to_string(),
        Custom => return "test3".to_string(),
    }
}
