use std::char;

use chumsky::prelude::*;

#[derive(Debug)]
pub enum Token {
    Label(String),
    Number(u32),
    Register(String),

    Mnemonic(String),
    Modifier(String),
    FullMnemonic {
        mnemonic: Box<Token>,
        modifiers: Vec<Token>,
    },

    Parameter(Box<Token>),

    Operation {
        full_mnemonic: Box<Token>,
        parameters: Vec<Token>,
    },

    Assembly(Vec<Token>),
}

#[derive(Debug)]
enum Reg {
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

#[derive(Debug)]
enum Mnemonic {
    And,
    Or,
    Xor,
    Not,
    Add,
    Addc,
    Sub,
    Subb,
    Shl,
    Ashl,
    Shr,
    Ld,
    St,
    Push,
    Pop,
}

#[derive(Debug)]
enum Modifier {}

pub fn parser() -> impl Parser<char, Token, Error = Simple<char>> {
    let label = text::ident()
        .then_ignore(just(':'))
        .map(|s: String| Token::Label(s));

    let hex_num = just("#0x")
        .ignore_then(text::int(16).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    let dec_num =
        just("#").ignore_then(text::int(10).map(|s: String| u32::from_str_radix(&s, 10).unwrap()));

    let number = hex_num
        .clone()
        .or(dec_num.clone())
        .padded()
        .map(|n: u32| Token::Number(n));

    let mnemonic = text::ident().padded().map(|s: String| Token::Mnemonic(s));
    let register = text::ident().padded().map(|s: String| Token::Register(s));

    let modifier = just('.').ignore_then(text::ident().map(|s: String| Token::Modifier(s)));

    let full_mnemonic =
        mnemonic
            .clone()
            .then(modifier.clone().repeated())
            .map(|(mnemonic, modifiers)| Token::FullMnemonic {
                mnemonic: Box::new(mnemonic),
                modifiers,
            });

    let parameter = number.clone().or(register.clone());

    let operation = full_mnemonic
        .clone()
        .then(parameter.clone().separated_by(just(',')))
        .then_ignore(just(';'))
        .map(|(full_mnemonic, parameters)| Token::Operation {
            full_mnemonic: Box::new(full_mnemonic),
            parameters,
        });

    let assembly = (operation.clone().or(label.clone()).padded())
        .repeated()
        .map(|operations| Token::Assembly(operations));

    return assembly.then_ignore(end());
}
