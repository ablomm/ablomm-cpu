use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use ariadne::{sources, Cache, Fmt};
use ast::{Ast, Block, File, Import, Literal, Operation, Spanned, Statement};
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

    let mut file_exports_map = HashMap::new();
    for file in &file_queue {
        // can't do map_err because of borrow checker
        match fill_symbol_table(
            &file.src,
            &Spanned::new(&file.block, file.span),
            file.start_address,
            &mut file_exports_map,
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

    // add imports to end of this file
    end_address += file.block.num_words();

    // after finding correct addresses
    for import in file.block.get_imports() {
        // conanicalize path
        let import_src = match src
            .parent()
            .unwrap()
            .join(Path::new(&*import.file.val))
            .canonicalize()
        {
            Ok(path) => Spanned::new(path, import.file.span),
            Err(error) => return Err(vec![Error::new(error.to_string(), import.file.span)]),
        };

        let import_intern = Intern::new(import_src.to_str().unwrap().to_string());

        if import_set.contains(&import_intern) {
            // circular dependency
            return Err(vec![Error::new(
                format!(
                    "Circular dependency detected: '{}' (transitiviely) imports '{}' which imports '{}'",
                    import_src.file_name().unwrap().to_str().unwrap().fg(ATTENTION_COLOR),
                    src.file_name().unwrap().to_str().unwrap().fg(ATTENTION_COLOR),
                    import_src.file_name().unwrap().to_str().unwrap().fg(ATTENTION_COLOR)
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
    file_exports_map: &mut HashMap<PathBuf, HashMap<Intern<String>, u32>>,
) -> Result<(), Error> {
    let mut address: u32 = start_address;
    let mut export_identifiers = Vec::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                block
                    .symbol_table
                    .borrow_mut()
                    .try_insert(*label, address)
                    .map_err(|_| Error::new("Identifier already defined", statement.span))?;
            }
            Statement::Assignment(identifier, expression) => {
                // need to move the expression evaluation out of the symbol_table.insert() call
                // to satisfy the borrow checker
                let expression = expression.as_ref().eval(&block.symbol_table.borrow())?;

                block
                    .symbol_table
                    .borrow_mut()
                    .try_insert(identifier.val, expression)
                    .map_err(|_| Error::new("Identifier already defined", identifier.span))?;
            }
            Statement::Export(exports) => {
                for export in exports {
                    export_identifiers.push(export);
                }
            }
            Statement::Import(import) => {
                let import_src = src.parent().unwrap().join(Path::new(&*import.file.val));
                let file_exports = file_exports_map.get(&import_src).ok_or(Error::new(
                    "[Internal Error] Attempted to import when file has not yet been parsed",
                    import.file.span,
                ))?;
                match &import.identifiers {
                    // restricted import (i.e. import * [as *] from *
                    Some(idents) => {
                        for ident in &idents.val {
                            let import_val =
                                file_exports.get(&ident.identifier).ok_or(Error::new(
                                    format!(
                                        "Identifier is not exported in '{}'",
                                        import.file.val.as_str().fg(ATTENTION_COLOR)
                                    ),
                                    ident.identifier.span,
                                ))?;

                            let import_key = &ident.alias.as_ref().unwrap_or(&ident.identifier);

                            block
                                .symbol_table
                                .borrow_mut()
                                .try_insert(import_key.val, *import_val)
                                .map_err(|_| {
                                    Error::new("Identifier already defines", import_key.span)
                                })?;
                        }
                    }
                    None => {
                        // unrestricted import (i.e. import from *)
                        for (import_key, import_val) in file_exports {
                            block.symbol_table.borrow_mut().try_insert(*import_key, *import_val).map_err(|_| Error::new(
                            format!("Import contains identifier '{}', which already exists in this scope", import_key.fg(ATTENTION_COLOR)),
                            import.file.span,
                    ))?;
                        }
                    }
                }
            }
            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));
                fill_symbol_table(
                    src,
                    &Spanned::new(sub_block, statement.span),
                    address,
                    file_exports_map,
                )?;
            }
            _ => (),
        }

        // technically we could count the lines in the loop above, but this is a bit more readable
        // even though it requres another pass
        address += statement.num_words();
    }

    // get exports from exports_map, create one if it doesn't exist
    let exports = match file_exports_map.get_mut(src) {
        Some(exports) => exports,
        None => {
            let exports = HashMap::new();
            file_exports_map.insert(src.to_path_buf(), exports);

            file_exports_map.get_mut(src).unwrap()
        }
    };
    // could technically do exports in the statements loop, but then exports need to come after
    // assignments in the assembly code
    for identifier in export_identifiers {
        if exports.contains_key(&identifier.val) {
            return Err(Error::new("Identifier already exported", identifier.span));
        }
        let export_val = get_identifier(&identifier.as_ref(), &block.symbol_table.borrow())?;
        exports.insert(identifier.val, export_val);
    }

    Ok(())
}

impl Literal {
    pub fn num_words(&self) -> u32 {
        match self {
            Literal::String(string) => ((string.len() as f32) / 4.0).ceil() as u32,
            _ => 1,
        }
    }
}

impl Operation {
    pub fn num_words(&self) -> u32 {
        1
    }
}

impl Block {
    pub fn num_words(&self) -> u32 {
        let mut num_words = 0;
        for statement in &self.statements {
            num_words += statement.num_words();
        }

        num_words
    }

    pub fn get_imports(&self) -> Vec<&Import> {
        let mut imports = Vec::new();

        for statement in &self.statements {
            match &statement.val {
                Statement::Import(import) => imports.push(import),
                Statement::Block(block) => imports.append(&mut block.get_imports()),
                _ => (),
            }
        }

        imports
    }
}

impl Statement {
    pub fn num_words(&self) -> u32 {
        match self {
            Statement::Literal(literal) => literal.num_words(),
            Statement::Block(block) => block.num_words(),
            Statement::Operation(operation) => operation.num_words(),
            _ => 0,
        }
    }
}
