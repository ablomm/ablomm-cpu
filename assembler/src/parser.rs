use crate::{error::*, Span};
use chumsky::prelude::*;
use std::{char, ops::Deref};
use text::TextParser;

pub fn parser() -> impl Parser<char, Vec<Statement>, Error = Error> {
    let label = text::ident();

    // supports leading 0s (e.g. 0b0010, 0x00fff, 0o00300, 000123)
    let bin_num = just("0b")
        .ignore_then(just("0").repeated())
        .ignore_then(text::int(2).map(|s: String| u32::from_str_radix(&s, 2).unwrap()));
    let oct_num = just("0o")
        .ignore_then(just("0").repeated())
        .ignore_then(text::int(8).map(|s: String| u32::from_str_radix(&s, 8).unwrap()));
    let hex_num = just("0x")
        .ignore_then(just("0").repeated())
        .ignore_then(text::int(16).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    let dec_num = just("0")
        .repeated()
        .ignore_then(text::int(10))
        .map(|s: String| u32::from_str_radix(&s, 10).unwrap());

    // no need to escape ' or \ since ' and \ can be represented by ''' and '\'
    // we're able to do that because empty chars ('') are not supported
    let escape_char = just('\\').ignore_then(choice((
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
        just('0').to('\0'),
    )));

    let char_num = escape_char
        .or(any())
        .delimited_by(just('\''), just('\''))
        .map(|c| c as u32);

    let pos_number = choice((bin_num, oct_num, hex_num, dec_num, char_num));

    // negative numbers cause some unintuitive behaviour due to the limited bit length of
    // immediates with no sign extension. Looks like other assemblers handles negative numbers by
    // simply converting the operation into the equivalent with unsigned numbers 
    // (e.g. "sub r1, -1;" turns into "sub r1, 1;")
    // let neg_number = just("-").ignore_then(pos_number).map(|num| -(num as i32));
    // let number = pos_number.or(neg_number.map(|num| num as u32));

    let number = pos_number;

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
        .map_with_span(Spanned::new)
        .then(
            modifier
                .map_with_span(Spanned::new)
                .repeated()
                .map_with_span(Spanned::new),
        )
        .map(|(mnemonic, modifiers)| FullMnemonic {
            mnemonic,
            modifiers,
        });

    let operation = full_mnemonic
        .map_with_span(Spanned::new)
        .padded()
        .then(
            parameter
                .map_with_span(Spanned::new)
                .padded()
                .separated_by(just(','))
                .map_with_span(Spanned::new),
        )
        .map(|(full_mnemonic, parameters)| Operation {
            full_mnemonic,
            parameters,
        });

    let line_comment = just("//").ignore_then(take_until(just("\n")).padded());
    let multiline_comment = just("/*").ignore_then(take_until(just("*/")).padded());
    let comment = line_comment.or(multiline_comment);

    let statement = choice((
        operation
            .then_ignore(just(';'))
            .map_with_span(Spanned::new)
            .map(Statement::Operation),
        label
            .then_ignore(just(':'))
            .map_with_span(Spanned::new)
            .map(Statement::Label),
        comment
            .map_with_span(|(_, comment), span| Spanned::new(comment.into(), span))
            .map(Statement::Comment),
    ))
    .padded();

    return statement.repeated().then_ignore(end());
}

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
    Operation(Spanned<Operation>),
    Label(Spanned<String>),
    Comment(Spanned<String>), // added because maybe it will be useful some day
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
    PASSA = 0,
    PASSB,
    AND,
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
    LD = 0x10,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP,
    INT,
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
    Label(String),
    Number(u32),
    Register(Register),
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
    R11,
    FP,
    STATUS,
    SP,
    PC,
}
