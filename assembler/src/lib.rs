use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ast::{Block, SymbolTable};
use chumsky::prelude::*;

use error::*;
use generator::*;
use internment::Intern;
use parser::*;
use span::*;

mod ast;
pub mod error;
mod generator;
mod parser;
mod span;

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

    // make life simple; a file is a block
    let ast = Block {
        statements: ast,
        symbol_table: Rc::new(RefCell::new(SymbolTable {
            table: HashMap::new(),
            parent: None,
        })),
    };

    return generate(ast).map_err(|error| vec![error]);
}
