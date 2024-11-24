use crate::{error::*, Span};
use chumsky::prelude::*;
use std::{char, ops::Deref};
use text::TextParser;

pub fn expression_parser() -> impl Parser<char, Expression, Error = Error> {
    let bin_num = just("0b")
        .ignore_then(text::digits(2).map(|s: String| u32::from_str_radix(&s, 2).unwrap()));
    let oct_num = just("0o")
        .ignore_then(text::digits(8).map(|s: String| u32::from_str_radix(&s, 8).unwrap()));
    let hex_num = just("0x")
        .ignore_then(text::digits(16).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    let dec_num = text::digits(10).map(|s: String| u32::from_str_radix(&s, 10).unwrap());

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
    // (e.g. "add r1, -1;" turns into "sub r1, 1;")
    // let neg_number = just("-").ignore_then(pos_number).map(|num| -(num as i32));
    // let number = pos_number.or(neg_number.map(|num| num as u32));

    let atom = choice((
        pos_number.map(Expression::Number),
        text::ident().map(Expression::Ident),
    ));

    let unary = just('-')
        .padded()
        .repeated()
        .then(atom)
        .foldr(|_op, rhs| Expression::Neg(Box::new(rhs)));

    let product = unary
        .then(
            choice((
                just('*').padded().to(Expression::Mul as fn(_, _) -> _),
                just('/').padded().to(Expression::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

    let sum = product
        .then(
            choice((
                just('+').padded().to(Expression::Add as fn(_, _) -> _),
                just('-').padded().to(Expression::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

    return sum;
}

pub fn parser() -> impl Parser<char, Vec<Spanned<Statement>>, Error = Error> {
    let label = text::ident();
    // no need to escape ' or \ since ' and \ can be represented by ''' and '\'
    // we're able to do that because empty chars ('') are not supported
    let escape_char = just('\\').ignore_then(choice((
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
        just('0').to('\0'),
    )));

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
        text::keyword("fp").to(Register::FP),
        text::keyword("status").to(Register::STATUS),
        text::keyword("sp").to(Register::SP),
        text::keyword("lr").to(Register::LR),
        text::keyword("pc").to(Register::PC),
    ));

    let parameter = recursive(|parameter| {
        let indirect = parameter.delimited_by(just('['), just(']'));
        return choice((
            register.map(Parameter::Register),
            expression_parser().map(Parameter::Expression),
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
        text::keyword("nop").to(Mnemonic::NOP),
        text::keyword("ld").to(Mnemonic::LD),
        text::keyword("st").to(Mnemonic::ST),
        text::keyword("push").to(Mnemonic::PUSH),
        text::keyword("pop").to(Mnemonic::POP),
        text::keyword("int").to(Mnemonic::INT),
        text::keyword("and").to(Mnemonic::AND),
        text::keyword("or").to(Mnemonic::OR),
        text::keyword("xor").to(Mnemonic::XOR),
        text::keyword("not").to(Mnemonic::NOT),
        text::keyword("add").to(Mnemonic::ADD),
        text::keyword("addc").to(Mnemonic::ADDC),
        text::keyword("sub").to(Mnemonic::SUB),
        text::keyword("subb").to(Mnemonic::SUBB),
        text::keyword("neg").to(Mnemonic::NEG),
        text::keyword("shl").to(Mnemonic::SHL),
        text::keyword("shr").to(Mnemonic::SHR),
        text::keyword("ashr").to(Mnemonic::ASHR),
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

    let string = filter(|c| *c != '\\' && *c != '\"')
        .or(escape_char)
        .repeated()
        .collect::<String>()
        .delimited_by(just("\""), just("\""));

    let literal = choice((
        expression_parser().map(Literal::Expression),
        string.map(Literal::String),
    ));

    let statement = choice((
        operation.then_ignore(just(';')).map(Statement::Operation),
        label.then_ignore(just(':')).map(Statement::Label),
        literal.then_ignore(just(';')).map(Statement::Literal),
        comment.map(|(_, comment)| Statement::Comment(comment.into())),
    ))
    .padded();

    return statement
        .map_with_span(Spanned::new)
        .repeated()
        .then_ignore(end());
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
    Neg(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
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
