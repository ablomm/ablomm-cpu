use chumsky::prelude::*;

use error::*;
use generator::*;
use internment::Intern;
use parser::*;
use span::*;

pub mod error;
mod generator;
mod parser;
mod span;
mod ast;

pub fn assemble(assembly_code: &str, src: Intern<String>) -> Result<String, Vec<Error>> {
    let len = assembly_code.chars().count();
    let eoi = Span::new(src, len..len);

    let ast = parser().parse(chumsky::Stream::from_iter(
        eoi,
        assembly_code
            .chars()
            .enumerate()
            .map(|(i, c)| (c, Span::new(src, i..i + 1))),
    ))?;

    return generate(ast).map_err(|error| vec![error]);
}
