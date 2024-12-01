use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use ariadne::{sources, Cache};
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

pub fn assemble(src: &String) -> Result<String, (Vec<Error>, impl Cache<Intern<String>>)> {
    assert!(Path::new(src).exists(), "file: '{}' does not exist", src);
    // should never fail?
    let src = Path::new(src).canonicalize().expect("Error finding file");
    let mut statements = Vec::new();
    let mut cache = HashMap::new();
    let mut import_stack = VecDeque::new();

    // create a dummy span because the assert before ""should"" guarantee no errors for that span
    let dummy_span = Span::new(Intern::new(src.to_str().unwrap().to_string()), 0..0);
    import_stack.push_back(Spanned::new(src, dummy_span));

    while let Some(src) = import_stack.pop_front() {
        // conanicalize path
        let src = match src.canonicalize() {
            Ok(path) => Spanned::new(path, src.span),
            Err(error) => {
                return Err((
                    vec![Error::new(error.to_string(), src.span)],
                    sources(cache.into_iter()),
                ))
            }
        };
        
        // parse file
        let block = match parse_path(&src.as_ref(), &mut cache) {
            Ok(block) => block,
            Err(err) => return Err((err, sources(cache.into_iter()))),
        };

        // add imports
        find_imports(&block.as_ref())
            .into_iter()
            .filter(|import| !cache.contains_key(&import.val))
            .for_each(|import| {
                import_stack.push_back(Spanned::new(
                    src.parent().unwrap().join(Path::new(&*import.val)),
                    import.span,
                ))
            });
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

    compile_ast(&ast).map_err(|error| (vec![error], sources(cache.into_iter())))
}

fn find_imports(block: &Spanned<&Block>) -> Vec<Spanned<Intern<String>>> {
    let mut imports = Vec::new();

    for statement in &block.statements {
        if let Statement::Import(import) = &statement.val {
            imports.push(Spanned::new(Intern::new(import.val.clone()), import.span));
        }
    }

    imports
}
/*
fn find_imports(
    block: &Spanned<&Block>,
    statements_stack: &mut VecDeque<Statement>,
) -> Vec<Spanned<Intern<String>>> {
    for statement in &block.statements {
        if let Statement::Import(import) = &statement.val {
            imports.push(Spanned::new(Intern::new(import.val.clone()), import.span));
        }
    }

    imports
}
*/

fn parse_path(
    path: &Spanned<&PathBuf>,
    cache: &mut HashMap<Intern<String>, String>,
) -> Result<Spanned<Block>, Vec<Error>> {
    let intern = Intern::new(path.to_str().unwrap().to_string());
    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
    let assembly_code = match fs::read_to_string(&*path.val) {
        Ok(assembly_code) => assembly_code,
        Err(err) => return Err(vec![Error::new(err.to_string(), path.span)]),
    };

    cache.insert(intern, assembly_code);
    let assembly_code = cache.get(&intern).unwrap(); // can unwrap since we just inserted
    let len = assembly_code.chars().count();
    let eoi = Span::new(intern, len..len);

    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
    let block = match parser().parse(chumsky::Stream::from_iter(
        eoi,
        assembly_code
            .chars()
            .enumerate()
            .map(|(i, c)| (c, Span::new(intern, i..i + 1))),
    )) {
        Ok(block) => block,
        Err(err) => return Err(err),
    };

    Ok(block)
}
