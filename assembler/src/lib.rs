use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    path::Path,
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
        let intern = Intern::new(src.to_str().unwrap().to_string());
        if cache.contains_key(&intern) {
            // already imported this file
            continue;
        }

        // need to do a match here because map_err causes the borrow checker to think that cache is
        // moved into the map_err closure
        let assembly_code = match fs::read_to_string(&*src.val) {
            Ok(assembly_code) => assembly_code,
            Err(err) => {
                return Err((
                    vec![Error::new(err.to_string(), src.span)],
                    sources(cache.into_iter()),
                ))
            }
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
            Err(err) => return Err((err, sources(cache.into_iter()))),
        };

        for import in find_imports(&block.as_ref())
            .into_iter()
            .filter(|import_src| !cache.contains_key(&import_src.val))
        {
            import_stack.push_back(Spanned::new(
                match src
                    .parent()
                    .unwrap()
                    .join(Path::new(&*import.val))
                    .canonicalize()
                {
                    Ok(path) => path,
                    Err(err) => {
                        return Err((
                            vec![Error::new(err.to_string(), import.span)],
                            sources(cache.into_iter()),
                        ))
                    }
                },
                import.span,
            ))
        }
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

    return compile_ast(&ast).map_err(|error| (vec![error], sources(cache.into_iter())));
}

pub fn find_imports(block: &Spanned<&Block>) -> Vec<Spanned<Intern<String>>> {
    let mut imports = Vec::new();

    for statement in &block.statements {
        if let Statement::Import(import) = &statement.val {
            imports.push(Spanned::new(Intern::new(import.val.clone()), import.span));
        }
    }
    return imports;
}
