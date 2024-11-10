use chumsky::prelude::*;

use error::*;
use generator::*;
use internment::Intern;
use parser::*;
use span::*;

pub mod error;
pub mod generator;
pub mod parser;
pub mod span;

pub fn assemble(assembly_code: &str, src: Intern<String>) -> Result<String, Vec<Error>> {
    let len = assembly_code.chars().count();
    let eoi = Span::new(src, len..len);

    match parser().parse(chumsky::Stream::from_iter(
        eoi,
        assembly_code
            .chars()
            .enumerate()
            .map(|(i, c)| (c, Span::new(src, i..i + 1))),
    )) {
        Ok(ast) => match generate(ast) {
            Ok(machine_code) => Ok(machine_code),
            Err(error) => return Err(vec![error]),
        },
        Err(error) => {
            return Err(error);
        }
    }
}
