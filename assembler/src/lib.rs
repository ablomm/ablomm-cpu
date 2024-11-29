use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fs,
    rc::Rc,
};

use ariadne::sources;
use ast::{Block, Spanned, Statement};
use chumsky::prelude::*;

use error::*;
use generator::*;
use internment::Intern;
use parser::*;
use span::*;
use symbol_table::SymbolTable;

mod ast;
pub mod error;
mod generator;
mod parser;
mod span;
mod symbol_table;

pub fn assemble(src: &String) -> Result<String, Vec<Error>> {
    let src = Intern::new(src.clone());
    let mut statements = Vec::new();
    let mut included_set = HashSet::new();
    let mut include_stack = VecDeque::new();
    include_stack.push_back(src);

    //let mut cache = sources(std::iter::once((src, &assembly_code)));

    while let Some(src) = include_stack.pop_front() {
        included_set.insert(src);

        let assembly_code = fs::read_to_string(&*src).expect("Error reading file");
        let len = assembly_code.chars().count();
        let eoi = Span::new(src, len..len);

        let block = parser().parse(chumsky::Stream::from_iter(
            eoi,
            assembly_code
                .chars()
                .enumerate()
                .map(|(i, c)| (c, Span::new(src, i..i + 1))),
        ))?;

        let includes = find_includes(&block.as_ref());
        includes
            .iter()
            .filter(|src| !included_set.contains(src))
            .for_each(|src| include_stack.push_back(*src));
        statements.push(Spanned::new(Statement::Block(block.val), block.span));
    }

    let root_symbol_table = Rc::new(RefCell::new(SymbolTable {
        table: HashMap::new(),
        parent: None,
    }));

    let ast = Block {
        statements,
        symbol_table: root_symbol_table,
    };

    return compile_ast(&ast).map_err(|error| vec![error]);
}
