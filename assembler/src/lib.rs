use chumsky::prelude::*;

use generator::*;
use internment::Intern;
use parser::*;
use span::*;
use error::*;

pub mod error;
pub mod generator;
pub mod parser;
pub mod span;

pub fn assemble(assembly: &str, src: Intern<String>) -> Result<String, Vec<Error>> {
    let len = assembly.chars().count();
    let eoi = Span::new(src, len..len);

    match parser().parse(chumsky::Stream::from_iter(
        eoi,
        assembly
            .chars()
            .enumerate()
            .map(|(i, c)| (c, Span::new(src, i..i + 1))),
    )) {
        Ok(ast) => {
            match generate(ast) {
                Ok(machine_code) => Ok(machine_code),
                Err(error) => panic!("{}", error),
            }
        }
        Err(error) => {
            return Err(error);
        }
    }
}
