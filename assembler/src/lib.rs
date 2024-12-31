use std::{collections::HashMap, fs, io, path::Path, rc::Rc};

use ariadne::{sources, Cache, Fmt};
use ast::{Ast, Block, Expression, File, Import, ImportSpecifier, Operation, Spanned, Statement};
use chumsky::prelude::*;

use error::*;
use expression::{
    expression_result::{ExpressionResult, Number},
    EvalReturn,
};
use generator::*;
use indexmap::IndexMap;
use internment::Intern;
use parser::*;
use span::*;
use src::Src;
use symbol_table::{get_identifier, STEntry, SymbolTable};

mod ast;
pub mod error;
mod expression;
mod generator;
mod parser;
mod span;
mod src;
mod symbol_table;

// return a string which is the machine code
// error includes cache in order to print errors without re-reading files
pub fn assemble(src: &String) -> Result<String, (Vec<Error>, impl Cache<Intern<Src>>)> {
    // fails if file not found
    // this is the root file, given in the comandline argument
    let src = Intern::new(
        Src::new(Path::new(src).to_path_buf())
            .unwrap_or_else(|error| panic!("Error finding file '{}': {}", src, error)),
    );

    // create a dummy span because there is no actual span for the root file, as it doesn't have a
    // corresponding import statement
    let dummy_span = Span::new(src, 0..0);
    let src = Spanned::new(src, dummy_span);

    // cache of file name and corresponding file contents, used to
    // print error messages and check if a file has already been parsed
    let mut cache = HashMap::new();

    // file queue is order in which to generate symbol tables
    // can't do map_err because of borrow checker
    let mut file_queue = match generate_file_queue(&src, &mut cache, &mut IndexMap::new()) {
        Ok(file_queue) => file_queue,
        Err(error) => return Err((error, sources(cache))),
    };

    // needed to get types (and as much values as possible without addresses) because the number of
    // words for a statement may depend on the type and value of symbols
    match fill_symbol_tables(&file_queue) {
        Ok(_) => (),
        Err(error) => return Err((vec![error], sources(cache))),
    }

    // get file addresses (also technically does labels and assignments (but not imports / exports), but this is overwritten in the final
    // fill_symbol_tables pass)
    match calculate_addresses(&mut file_queue) {
        Ok(_) => (),
        Err(error) => return Err((vec![error], sources(cache))),
    }

    // final fill after all addresses are calculated, all values should be filled in this pass
    match fill_symbol_tables(&file_queue) {
        Ok(_) => (),
        Err(error) => return Err((vec![error], sources(cache))),
    }

    let ast = Ast {
        // reverse order to get correct generation order, which is opposite of symbol table
        // creation order (i.e. had to create imported file's symbol table before itself)
        files: file_queue.into_iter().rev().collect(),
    };

    compile_ast(&ast).map_err(|error| (error, sources(cache)))
}

// takes in a canonanical file path, address, and cache and returns a queue of the order in which to
// generate the symbol tables / exports in order to satisfy import dependencies (i.e. post order)
fn generate_file_queue(
    src: &Spanned<Intern<Src>>,
    cache: &mut HashMap<Intern<Src>, String>,
    import_map: &mut IndexMap<Intern<Src>, Span>, // to detect cycles, IndexSet to preserve insert order; useful for errors
) -> Result<Vec<Spanned<File>>, Vec<Error>> {
    import_map.insert(src.val, src.span);

    let mut file_queue = Vec::new();

    // parse itself
    let file = parse_path(src, cache)?;

    // after finding correct addresses
    for import in file.block.get_imports() {
        let import_src = match get_import_src(src, import) {
            Ok(import_src) => import.file.span_to(import_src),
            Err(error) => {
                return Err(vec![Error::new(import.file.span, "Error finding file")
                    .with_label(error.to_string())])
            }
        };

        if import_map.contains_key(&import_src.val) {
            // circular dependency
            let mut error = Error::new(import_src.span, "Circular dependency");

            // skipping one because the first one is always the root file, which has no
            // corresponding import statement
            for (_back_src, back_span) in import_map.iter().skip(1) {
                error = error.with_label_span(*back_span, "Imported here");
            }
            error = error.with_label("This completes the circle, causing a circular dependency");

            return Err(vec![error.with_help("Try removing one of these imports")]);
        }

        if cache.contains_key(&import_src.val) {
            // we already did this import, but in another branch (so not a ciruclar dependency), so skip it
            continue;
        }

        file_queue.append(&mut generate_file_queue(&import_src, cache, import_map)?);
    }

    // only add itself after imports are added (post order)
    file_queue.push(file);

    import_map.shift_remove(&src.val);
    Ok(file_queue)
}

// takes a path and parses it
fn parse_path(
    src: &Spanned<Intern<Src>>,
    cache: &mut HashMap<Intern<Src>, String>,
) -> Result<Spanned<File>, Vec<Error>> {
    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
    let assembly_code = fs::read_to_string(src.as_path()).map_err(|error| {
        vec![Error::new(src.span, "Error reading file").with_label(error.to_string())]
    })?;

    // need to insert before parsing so it can show parsing errors
    cache.insert(src.val, assembly_code);
    let assembly_code = cache.get(&src.val).unwrap(); // unwrap safe because we just inserted

    let len = assembly_code.chars().count();
    let eoi = Span::new(src.val, len..len);

    let block = parser().parse(chumsky::Stream::from_iter(
        eoi,
        assembly_code
            .chars()
            .enumerate()
            .map(|(i, c)| (c, Span::new(src.val, i..i + 1))),
    ))?;

    let file = File {
        src: src.val,
        start_address: None,
        block: block.val,
    };

    Ok(block.span.spanned(file))
}

fn fill_symbol_tables(file_queue: &Vec<Spanned<File>>) -> Result<(), Error> {
    let mut file_exports_map = HashMap::new();
    for file in file_queue {
        // can't do map_err because of borrow checker
        fill_symbol_table(
            &file.src,
            &file.span_to(&file.block),
            file.start_address,
            &mut file_exports_map,
        )?
    }
    Ok(())
}

// generates symbol table for block and sub_block
fn fill_symbol_table(
    src: &Intern<Src>,
    block: &Spanned<&Block>,
    // if start_address is None, it will simply fill the symbol_table to the best of it's ability
    // without any addresses
    start_address: Option<u32>,
    file_exports_map: &mut HashMap<Intern<Src>, HashMap<Intern<String>, STEntry>>,
) -> Result<(), Error> {
    // start from scratch to prevent duplicate errors (yes technically not as fast as it could be,
    // but much easier to reason about)
    block.symbol_table.borrow_mut().table.clear();
    let mut address = start_address;

    // this if statement makes the file_export_map.get_mut(src).unwrap() safe to unwrap
    if !file_exports_map.contains_key(src) {
        file_exports_map.insert(*src, HashMap::new());
    }

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                block
                    .symbol_table
                    .borrow_mut()
                    .try_insert(
                        label.identifier,
                        label
                            .identifier
                            .span_to(ExpressionResult::Number(address.map(Number))),
                    )
                    .map_err(|error| {
                        Error::new(label.identifier.span, "Identifier already defined")
                            .with_label("Defined again here")
                            .with_label_span(error.0.key_span, "Defined first here")
                            .with_help("Try using a different name")
                    })?;
                if label.export {
                    export(
                        &label.identifier,
                        &block.symbol_table.borrow(),
                        file_exports_map.get_mut(src).unwrap(),
                    )?;
                }
            }

            Statement::Assignment(assignment) => {
                // need to move the expression evaluation out of the symbol_table.insert() call
                // to satisfy the borrow checker
                let value = assignment.expression.span_to(
                    assignment
                        .expression
                        .as_ref()
                        .eval(&block.symbol_table.borrow())?
                        .result,
                );

                block
                    .symbol_table
                    .borrow_mut()
                    .try_insert(assignment.identifier, value)
                    .map_err(|error| {
                        Error::new(assignment.identifier.span, "Identifier already defined")
                            .with_label("Defined again here")
                            .with_label_span(error.0.key_span, "Defined first here")
                            .with_help("Try using a different name")
                    })?;

                if assignment.export {
                    export(
                        &assignment.identifier,
                        &block.symbol_table.borrow(),
                        file_exports_map.get_mut(src).unwrap(),
                    )?;
                }
            }

            Statement::Export(identifiers) => {
                for identifier in identifiers {
                    export(
                        identifier,
                        &block.symbol_table.borrow(),
                        file_exports_map.get_mut(src).unwrap(),
                    )?;
                }
            }

            Statement::Import(import_val) => {
                let import_src = get_import_src(src, import_val).unwrap_or_else(|error| {
                    // should not occur because should already have gotten this import in
                    // generate_file_quueu()
                    panic!(
                        "Could not find import '{}' in file '{}': {}",
                        import_val.file.val, src, error
                    )
                });

                // the import files' exports
                let exports = file_exports_map.get(&import_src).unwrap_or_else(||
                    // should not occur because the order should ensure all dependencies are parsed
                    panic!(
                        "Attempted to import '{}' when the importee's symbol table has not been filled",
                        import_src
                    ));

                import(import_val, &mut block.symbol_table.borrow_mut(), exports)?
            }

            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));
                fill_symbol_table(
                    src,
                    &statement.span_to(sub_block),
                    address,
                    file_exports_map,
                )?;
            }
            _ => (),
        }

        // technically we could count the lines in the loop above, but this is a bit more readable
        // even though it requres another pass
        if let Some(mut address_val) = address {
            address_val += statement.as_ref().num_words(&block.symbol_table.borrow())?;
            address = Some(address_val);
        }
    }

    Ok(())
}

// needed because some future imports symbols depend on the size of the importer's length (yeah, I
// know, it's confusing)
// assume file_queue in in post_order
fn calculate_addresses(file_queue: &mut [Spanned<File>]) -> Result<(), Error> {
    let mut address = 0;
    for file in file_queue.iter_mut().rev() {
        file.val.start_address = Some(address);
        for statment in &file.block.statements {
            match &statment.val {
                // we now are able to calculate labels, which gives more values that can be used in
                // num_words()
                Statement::Label(label) => {
                    file.block.symbol_table.borrow_mut().insert(
                        label.identifier,
                        label
                            .identifier
                            .span_to(ExpressionResult::Number(Some(Number(address)))),
                    );
                }

                // now that the labels are known, we need to re-evaluate assignments, which may
                // depend on the label values
                // don't worry about exports, as the file_queue is in post_order which guarntees
                // all importers have already been parsed, so no one will use the new exports
                Statement::Assignment(assignment) => {
                    let value = assignment.expression.span_to(
                        assignment
                            .expression
                            .as_ref()
                            .eval(&file.block.symbol_table.borrow())?
                            .result,
                    );

                    file.block
                        .symbol_table
                        .borrow_mut()
                        .insert(assignment.identifier, value);
                }
                _ => (),
            }

            address += statment
                .as_ref()
                .num_words(&file.block.symbol_table.borrow())?;
        }
    }

    Ok(())
}

fn export(
    identifier: &Spanned<Intern<String>>,
    symbol_table: &SymbolTable,
    exports: &mut HashMap<Intern<String>, STEntry>,
) -> Result<(), Error> {
    if let Some(entry) = exports.get(&identifier.val) {
        return Err(Error::new(identifier.span, "Identifier already exported")
            .with_label("Exported again here")
            .with_label_span(entry.key_span, "Exported first here")
            .with_note("Try removing one of these exports"));
    }

    let export_val = get_identifier(&identifier.as_ref(), symbol_table)?;
    exports.insert(identifier.val, export_val);

    Ok(())
}

fn import(
    import: &Import,
    symbol_table: &mut SymbolTable,
    exports: &HashMap<Intern<String>, STEntry>,
) -> Result<(), Error> {
    match &import.specifier.val {
        // selective import (i.e. import <ident> [as <ident>][, ...] from <file>
        ImportSpecifier::Named(named_imports) => {
            for named_import in named_imports {
                let import_val = exports.get(&named_import.identifier).ok_or(
                    Error::new(named_import.identifier.span, "Identifier not found").with_label(
                        format!(
                            "Identifier is not exported in '{}'",
                            import.file.val.as_str().fg(ATTENTION_COLOR)
                        ),
                    ),
                )?;

                let import_key = named_import
                    .alias
                    .as_ref()
                    .unwrap_or(&named_import.identifier);

                symbol_table
                    .try_insert(*import_key, import_val.result.clone())
                    .map_err(|error| {
                        Error::new(import_key.span, "Identifier already defined")
                            .with_label("Defined again here")
                            .with_label_span(error.0.key_span, "Defined first here")
                            .with_help(format!(
                                "Try aliasing the import by adding {}",
                                "as <new_name>".fg(ATTENTION_COLOR)
                            ))
                    })?;
            }
        }

        // unselective import (i.e. import * from <file>
        ImportSpecifier::Blob => {
            for (import_key, import_val) in exports {
                symbol_table
                    .try_insert(
                        import_val.key_span.spanned(*import_key),
                        import_val.result.clone(),
                    )
                    .map_err(|error| {
                        Error::new(import.specifier.span, "Identifier already defined")
                            .with_label(format!(
                                "Import contains identifier '{}', which already exists in this scope",
                                import_key.fg(ATTENTION_COLOR)
                            ))
                            .with_label_span(error.0.key_span, "Defined first here")
                            .with_help(format!(
                                "Try using named import aliases: {}{}",
                                import_key.fg(ATTENTION_COLOR),
                                " as <new_name> ... <other imports>".fg(ATTENTION_COLOR)
                            ))
                    })?;
            }
        }
    }

    Ok(())
}

fn get_import_src(importer: &Intern<Src>, import: &Import) -> io::Result<Intern<Src>> {
    Ok(Intern::new(
        importer.get_relative(Path::new(&import.file.val))?,
    ))
}

impl Block {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, Error> {
        let mut num_words = 0;
        for statement in &self.statements {
            num_words += statement.as_ref().num_words(symbol_table)?;
        }

        Ok(num_words)
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

impl Spanned<&Statement> {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, Error> {
        match self.val {
            Statement::Literal(literal) => self.span_to(literal).num_words(symbol_table),
            Statement::Block(block) => block.num_words(symbol_table),
            Statement::Operation(operation) => operation.num_words(),
            _ => Ok(0),
        }
    }
}

impl Spanned<&Expression> {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, Error> {
        let EvalReturn {
            result,
            waiting_map,
        } = self.eval(symbol_table)?;

        match result {
            ExpressionResult::String(string) => {
                let string = string.ok_or_else(|| {
                    let mut error = Error::new(self.span, "Unknown value of expression").with_label(
                        "Expression's value is required at this point, but it is indetermined",
                    );
                    for span in waiting_map.values() {
                        error = error.with_label_span(*span, "This identifier has an unknown value")
                    }

                    error.with_note(
                        "This is ultimately caused because it is dependent on a future address (label), but the value of the expression would effect that address (label)",
                    )
                })?;

                Ok(((string.len() as f32) / 4.0).ceil() as u32)
            }
            ExpressionResult::Number(_number) => Ok(1),
            _ => Err(Error::new(self.span, "Incorrect type").with_label(format!(
                "Expected a {} or {}",
                "string".fg(ATTENTION_COLOR),
                "number".fg(ATTENTION_COLOR)
            ))),
        }
    }
}

impl Operation {
    pub fn num_words(&self) -> Result<u32, Error> {
        Ok(1)
    }
}
