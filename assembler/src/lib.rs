use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use ariadne::{sources, Cache};
use ast::{Ast, Block, File, Spanned, Statement};
use chumsky::prelude::*;

use error::*;
use generator::*;
use internment::Intern;
use parser::*;
use span::*;
use symbol_table::get_identifier;

mod ast;
pub mod error;
mod expression;
mod generator;
mod parser;
mod span;
mod symbol_table;

// return a string which is the machine code
// error includes cache in order to print errors without re-reading files
pub fn assemble(src: &String) -> Result<String, (Vec<Error>, impl Cache<Intern<String>>)> {
    // fails if file not found
    // this is the root file, given in the comandline
    let src = Path::new(src)
        .canonicalize()
        .expect("Error canonicalising file");

    let mut cache = HashMap::new(); // cache of file name and corresponding file contents, used to
                                    // print error messages and check if a file has already been parsed

    // create a dummy span because there is no actual span for the root file, as it doesn't have a
    // corresponding import statement
    let dummy_span = Span::new(Intern::new(src.to_str().unwrap().to_string()), 0..0);
    let src = Spanned::new(src, dummy_span);

    // file queue is order in which to generate symbol tables
    let file_queue = match generate_file_queue(src, 0, &mut cache, &mut HashSet::new()) {
        Ok(file_queue) => file_queue,
        Err(error) => return Err((error, sources(cache.into_iter()))),
    };

    let mut export_map = HashMap::new(); // holds every files' path along with it's exported
                                         // symbols
    for file in &file_queue {
        // can't do map_err because of borrow checker
        match fill_symbol_table(
            &file.src,
            &Spanned::new(&file.block, file.span),
            file.start_address,
            &mut export_map,
        ) {
            Ok(_address) => (),
            Err(error) => return Err((vec![error], sources(cache.into_iter()))),
        }
    }

    let ast = Ast {
        // reverse order to get correct generation order, which is opposite of symbol table
        // creation order (i.e. had to create imported file's symbol table before itself)
        files: file_queue.into_iter().rev().collect(),
    };

    compile_ast(&ast).map_err(|error| (vec![error], sources(cache.into_iter())))
}

// first pass of ast
// takes in a canonanical file path, address, and cache and returns a queue of the order in which to
// generate the symbol tables / exports in order to satisfy import dependencies (i.e. post order)
fn generate_file_queue(
    src: Spanned<PathBuf>,
    start_address: u32,
    cache: &mut HashMap<Intern<String>, String>,
    import_set: &mut HashSet<Intern<String>>, // to detect cycles
) -> Result<Vec<Spanned<File>>, Vec<Error>> {
    let src_intern = Intern::new(src.to_str().unwrap().to_string());
    import_set.insert(src_intern);

    let mut end_address = start_address;
    let mut file_queue = Vec::new();

    // parse itself
    let file = parse_path(&src.as_ref(), start_address, cache)?;

    let mut imports = Vec::new();

    // find all imports and count addresses
    for statement in &file.block.statements {
        match &statement.val {
            Statement::Import(import) => imports.push(import),
            _ => (),
        }
        end_address += statement.num_lines();
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

        let import_intern = Intern::new(import_src.to_str().unwrap().to_string());

        if import_set.contains(&import_intern) {
            // circular dependency
            return Err(vec![Error::new(
                format!(
                    "Circular dependency detected: '{}' (transitiviely) imports '{}' which imports '{}'",
                    import_src.file_name().unwrap().to_str().unwrap(),
                    src.file_name().unwrap().to_str().unwrap(),
                    import_src.file_name().unwrap().to_str().unwrap()
                ),
                import_src.span,
            )]);
        }

        if cache.contains_key(&import_intern) {
            // we already did this import, so skip it
            continue;
        }

        file_queue.append(&mut generate_file_queue(
            import_src,
            end_address,
            cache,
            import_set,
        )?);
    }

    // only add itself after imports are added (post order)
    file_queue.push(file);

    import_set.remove(&src_intern);
    Ok(file_queue)
}

// takes a path and parses it
fn parse_path(
    path: &Spanned<&PathBuf>,
    start_address: u32,
    cache: &mut HashMap<Intern<String>, String>,
) -> Result<Spanned<File>, Vec<Error>> {
    let intern = Intern::new(path.to_str().unwrap().to_string());
    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
    let assembly_code = match fs::read_to_string(path.val) {
        Ok(assembly_code) => assembly_code,
        Err(err) => return Err(vec![Error::new(err.to_string(), path.span)]),
    };

    cache.insert(intern, assembly_code);
    let assembly_code = cache.get(&intern).unwrap(); // can unwrap since we just inserted
    let len = assembly_code.chars().count();
    let eoi = Span::new(intern, len..len);

    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
    let block = parser().parse(chumsky::Stream::from_iter(
        eoi,
        assembly_code
            .chars()
            .enumerate()
            .map(|(i, c)| (c, Span::new(intern, i..i + 1))),
    ))?;

    let file = File {
        src: path.val.to_path_buf(),
        start_address,
        block: block.val,
    };

    Ok(Spanned::new(file, block.span))
}

// second pass
// generates symbol table for block and sub_block
fn fill_symbol_table(
    src: &PathBuf,
    block: &Spanned<&Block>,
    start_address: u32,
    export_map: &mut HashMap<PathBuf, HashMap<Intern<String>, u32>>,
) -> Result<(), Error> {
    let mut address: u32 = start_address;
    let mut export_identifiers = Vec::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                if block.symbol_table.borrow().contains_key(label) {
                    return Err(Error::new("Identifier already defined", statement.span));
                }
                block.symbol_table.borrow_mut().insert(*label, address);
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
            Statement::Export(exports) => {
                for export in exports {
                    export_identifiers.push(export);
                }
            }
            Statement::Import(import) => {
                let import_src = src.parent().unwrap().join(Path::new(&*import.val));
                for (export_key, export_val) in export_map.get(&import_src).into_iter().flatten() {
                    if block.symbol_table.borrow().contains_key(export_key) {
                        return Err(Error::new(
                            format!("Import contains identifier '{}', which that already exists in this scope", export_key),
                            import.span,
                        ));
                    }
                    block
                        .symbol_table
                        .borrow_mut()
                        .insert(*export_key, *export_val);
                }
            }
            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));
                fill_symbol_table(
                    src,
                    &Spanned::new(sub_block, statement.span),
                    address,
                    export_map,
                )?;
            }
            _ => (),
        }
        address += statement.num_lines();
    }

    let mut exports = HashMap::new();
    // could technically do exports in the statements loop, but then exports need to come after
    // assignments in the assembly code
    for identifier in export_identifiers {
        if exports.contains_key(&identifier.val) {
            return Err(Error::new("Identifier already exported", identifier.span));
        }
        let export_val = get_identifier(&identifier.as_ref(), &block.symbol_table.borrow())?;
        exports.insert(identifier.val, export_val);
    }

    export_map.insert(src.to_path_buf(), exports);

    Ok(())
}
