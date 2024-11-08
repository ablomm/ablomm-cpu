use chumsky::prelude::*;

use generator::*;
use parser::*;

pub mod error;
pub mod generator;
pub mod parser;

pub fn assemble(assembly: &str) -> Result<String, &'static str> {
    match parser().parse(assembly) {
        Ok(ast) => generate(ast),
        Err(_error) => {
            return Err("Parse error");
        }
    }
}
