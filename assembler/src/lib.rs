use std::{
    cell::RefCell,
    collections::HashMap,
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
    let mut export_map = HashMap::new();
    let mut cache = HashMap::new();

    // create a dummy span because the assert before ""should"" guarantee no errors for that span
    let dummy_span = Span::new(Intern::new(src.to_str().unwrap().to_string()), 0..0);
    let src = Spanned::new(src, dummy_span);

    let file_queue = match generate_file_queue(src, 0, &mut cache) {
        Ok(file_queue) => file_queue,
        Err(error) => return Err((error, sources(cache.into_iter()))),
    };

    for (src, block, address) in file_queue {
        match pre_process(Some(src), &block, address, &mut export_map) {
            Ok(_address) => (),
            Err(error) => return Err((vec![error], sources(cache.into_iter()))),
        }

        statements.push(Spanned::new(Statement::Block(block.val), block.span));
    }

    let root_symbol_table = Rc::new(RefCell::new(SymbolTable {
        table: HashMap::new(),
        parent: None,
    }));

    let ast = Block {
        statements: statements.into_iter().rev().collect(),
        symbol_table: root_symbol_table,
    };

    compile_ast(&ast).map_err(|error| (vec![error], sources(cache.into_iter())))
}

// first pass
// takes in a file path, address, and cache and returns a queue of the order in which to
// generate the abstract syntax trees in order to satisfy import dependencies (i.e. post order)
fn generate_file_queue(
    src: Spanned<PathBuf>,
    start_address: u32,
    cache: &mut HashMap<Intern<String>, String>,
) -> Result<Vec<(PathBuf, Spanned<Block>, u32)>, Vec<Error>> {
    let mut end_address = start_address;
    let mut file_queue = Vec::new();

    // parse itself
    let block = parse_path(&src.as_ref(), cache)?;

    let mut imports = Vec::new();

    // find all imports and count addresses
    for statement in &block.statements {
        match &statement.val {
            Statement::Import(import) => imports.push(import),
            Statement::Operation(_) => {
                end_address += 1;
            }
            Statement::Literal(literal) => {
                end_address += literal.num_lines();
            }
            _ => (),
        }
    }

    // after finding correct addresses
    for import in imports {
        let import_src = Spanned::new(
            src.parent().unwrap().join(Path::new(&*import.val)),
            import.span,
        );

        // conanicalize path
        let import_src = match import_src.canonicalize() {
            Ok(path) => Spanned::new(path, import_src.span),
            Err(error) => return Err(vec![Error::new(error.to_string(), import_src.span)]),
        };

        let intern = Intern::new(import_src.to_str().unwrap().to_string());
        if cache.contains_key(&intern) {
            // we already did this import, so skip it
            continue;
        }

        file_queue.append(&mut generate_file_queue(import_src, end_address, cache)?);
    }

    // only add itself after imports are added
    file_queue.push((src.val, block, start_address));

    Ok(file_queue)
}

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

// second pass
// cannot include span because blocks may span multiple different files
fn pre_process(
    src: Option<PathBuf>,
    block: &Block,
    start_address: u32,
    export_map: &mut HashMap<PathBuf, HashMap<Intern<String>, u32>>,
) -> Result<u32, Error> {
    let mut line_number: u32 = start_address;
    let mut exports_list = Vec::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                if block.symbol_table.borrow().contains_key(label) {
                    return Err(Error::new("Identifier already defined", statement.span));
                }
                block.symbol_table.borrow_mut().insert(*label, line_number);
            }
            Statement::Assignment(identifier, expression) => {
                if block.symbol_table.borrow().contains_key(&identifier.val) {
                    return Err(Error::new("Identifier already defined", identifier.span));
                }

                // need to move the expression evaluation out of the symbol_table.insert() call
                // to satisfy the borrow checker
                let expression_val = expression.as_ref().eval(&block.symbol_table.borrow())?;

                block
                    .symbol_table
                    .borrow_mut()
                    .insert(identifier.val, expression_val);
            }
            Statement::Operation(_) => {
                line_number += 1;
            }
            Statement::Literal(literal) => {
                line_number += literal.num_lines();
            }
            Statement::Export(exports) => {
                for export in exports {
                    exports_list.push(export);
                }
            }
            Statement::Import(import) => {
                if let Some(src) = &src {
                    let import_src = src.parent().unwrap().join(Path::new(&*import.val));
                    for (export_key, export_val) in
                        export_map.get(&import_src).into_iter().flatten()
                    {
                        if block.symbol_table.borrow().contains_key(export_key) {
                            return Err(Error::new(
                                "Import contains identifier that already exists in this scope",
                                import.span,
                            ));
                        }
                        block
                            .symbol_table
                            .borrow_mut()
                            .insert(*export_key, *export_val);
                    }
                }
            }
            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));
                line_number = pre_process(
                    None,
                    &Spanned::new(sub_block, statement.span),
                    line_number,
                    export_map,
                )?;
            }
            _ => (),
        }
    }

    let mut exports = HashMap::new();
    // need to do after so that we can ensure all symbols are in symbol table to eval
    for export in exports_list {
        if exports.contains_key(&export.val) {
            return Err(Error::new("Identifier already exported", export.span));
        }
        let export_val = get_identifier(&export.as_ref(), &*block.symbol_table.borrow())?;
        exports.insert(export.val, export_val);
    }

    if let Some(src) = src {
        export_map.insert(src, exports);
    }

    Ok(line_number)
}
