use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::Path,
    rc::Rc,
};

use ariadne::{sources, Cache, Fmt};
use ast::{Ast, Block, Expression, File, Import, ImportSpecifier, Operation, Spanned, Statement};
use chumsky::prelude::*;

use error::*;
use expression::expression_result::{ExpressionResult, Number};
use generator::*;
use internment::Intern;
use parser::*;
use span::*;
use src::Src;
use symbol_table::{get_identifier, SymbolTable};

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
    // this is the root file, given in the comandline
    let src = Intern::new(
        Src::new(Path::new(src).to_path_buf())
            .unwrap_or_else(|error| panic!("Error finding file '{}': {}", src, error)),
    );

    // create a dummy span because there is no actual span for the root file, as it doesn't have a
    // corresponding import statement
    let dummy_span = Span::new(src, 0..0);
    let src = Spanned::new(src, dummy_span);

    let mut cache = HashMap::new(); // cache of file name and corresponding file contents, used to
                                    // print error messages and check if a file has already been parsed

    // file queue is order in which to generate symbol tables
    // can't do map_err because of borrow checker
    let file_queue = match generate_file_queue(&src, 0, &mut cache, &mut HashSet::new()) {
        Ok(file_queue) => file_queue,
        Err(error) => return Err((error, sources(cache))),
    };

    let mut file_exports_map = HashMap::new();
    for file in &file_queue {
        // can't do map_err because of borrow checker
        match fill_symbol_table(
            &file.src,
            &file.span_to(&file.block),
            file.start_address,
            &mut file_exports_map,
        ) {
            Ok(_address) => (),
            Err(error) => return Err((vec![error], sources(cache))),
        }
    }

    let ast = Ast {
        // reverse order to get correct generation order, which is opposite of symbol table
        // creation order (i.e. had to create imported file's symbol table before itself)
        files: file_queue.into_iter().rev().collect(),
    };

    compile_ast(&ast).map_err(|error| (vec![error], sources(cache)))
}

// first pass of ast
// takes in a canonanical file path, address, and cache and returns a queue of the order in which to
// generate the symbol tables / exports in order to satisfy import dependencies (i.e. post order)
fn generate_file_queue(
    src: &Spanned<Intern<Src>>,
    start_address: u32,
    cache: &mut HashMap<Intern<Src>, String>,
    import_set: &mut HashSet<Intern<Src>>, // to detect cycles
) -> Result<Vec<Spanned<File>>, Vec<Error>> {
    import_set.insert(src.val);

    let mut end_address = start_address;
    let mut file_queue = Vec::new();

    // parse itself
    let file = parse_path(src, start_address, cache)?;

    // need to create an empty symbol table because a block has expressions, which could contain
    // any value, and literals are expressions, which means a to know the size of a block
    // requires knowing the value of symbols, but the symbol tables are filled in later, so this is
    // an issue. Right now lets just ignore it by disallowing identifiers in literals
    let dummy_table = SymbolTable {
        table: HashMap::new(),
        parent: None,
    };

    // add imports to end of this file
    end_address += file.block.num_words(&dummy_table).map_err(|err| {
        vec![err.with_note("Currently, identifiers is not supported in this statement")]
    })?;

    // after finding correct addresses
    for import in file.block.get_imports() {
        let import_src = match get_import_src(src, import) {
            Ok(import_src) => import.file.span_to(import_src),
            Err(error) => return Err(vec![Error::new(error.to_string(), import.file.span)]),
        };

        if import_set.contains(&import_src.val) {
            // circular dependency
            return Err(vec![Error::new(
                format!(
                    "Circular dependency detected: '{}' (transitiviely) imports '{}' which imports '{}'",
                    import_src.fg(ATTENTION_COLOR),
                    src.fg(ATTENTION_COLOR),
                    import_src.fg(ATTENTION_COLOR)
                ),
                import_src.span,
            ).with_note("Try removing this import")]);
        }

        if cache.contains_key(&import_src.val) {
            // we already did this import, but in another branch (so not a ciruclar dependency), so skip it
            continue;
        }

        file_queue.append(&mut generate_file_queue(
            &import_src,
            end_address,
            cache,
            import_set,
        )?);
    }

    // only add itself after imports are added (post order)
    file_queue.push(file);

    import_set.remove(&src.val);
    Ok(file_queue)
}

// takes a path and parses it
fn parse_path(
    src: &Spanned<Intern<Src>>,
    start_address: u32,
    cache: &mut HashMap<Intern<Src>, String>,
) -> Result<Spanned<File>, Vec<Error>> {
    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
    let assembly_code = fs::read_to_string(src.as_path())
        .map_err(|error| vec![Error::new(error.to_string(), src.span)])?;

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
        start_address,
        block: block.val,
    };

    Ok(block.span.spanned(file))
}

// second pass
// generates symbol table for block and sub_block
fn fill_symbol_table(
    src: &Intern<Src>,
    block: &Spanned<&Block>,
    start_address: u32,
    file_exports_map: &mut HashMap<Intern<Src>, HashMap<Intern<String>, Spanned<ExpressionResult>>>,
) -> Result<(), Error> {
    let mut address: u32 = start_address;

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
                        label.identifier.val,
                        label
                            .identifier
                            .span_to(ExpressionResult::Number(Number(address))),
                    )
                    .map_err(|_| {
                        Error::new("Identifier already defined", label.identifier.span)
                            .with_note("Try using a different name")
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
                        .eval(&block.symbol_table.borrow())?,
                );

                block
                    .symbol_table
                    .borrow_mut()
                    .try_insert(assignment.identifier.val, value)
                    .map_err(|_| {
                        Error::new("Identifier already defined", assignment.identifier.span)
                            .with_note("Try using a different name")
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
        address += statement.as_ref().num_words(&block.symbol_table.borrow())?;
    }

    Ok(())
}

fn export(
    identifier: &Spanned<Intern<String>>,
    symbol_table: &SymbolTable,
    exports: &mut HashMap<Intern<String>, Spanned<ExpressionResult>>,
) -> Result<(), Error> {
    if exports.contains_key(&identifier.val) {
        return Err(Error::new("Identifier already exported", identifier.span)
            .with_note("Try removing this export"));
    }

    let export_val = get_identifier(&identifier.as_ref(), symbol_table)?;
    exports.insert(identifier.val, export_val);

    Ok(())
}

fn import(
    import: &Import,
    symbol_table: &mut SymbolTable,
    exports: &HashMap<Intern<String>, Spanned<ExpressionResult>>,
) -> Result<(), Error> {
    match &import.specifier.val {
        // selective import (i.e. import <ident> [as <ident>][, ...] from <file>
        ImportSpecifier::Named(named_imports) => {
            for named_import in named_imports {
                let import_val = exports.get(&named_import.identifier).ok_or(Error::new(
                    format!(
                        "Identifier is not exported in '{}'",
                        import.file.val.as_str().fg(ATTENTION_COLOR)
                    ),
                    named_import.identifier.span,
                ))?;

                let import_key = &named_import
                    .alias
                    .as_ref()
                    .unwrap_or(&named_import.identifier);

                symbol_table
                    .try_insert(import_key.val, import_val.clone())
                    .map_err(|_| {
                        Error::new("Identifier already defined", import_key.span).with_note(
                            format!(
                                "Try aliasing the import by adding {}",
                                "as <new_name>".fg(ATTENTION_COLOR)
                            ),
                        )
                    })?;
            }
        }

        // unselective import (i.e. import * from <file>
        ImportSpecifier::Blob => {
            for (import_key, import_val) in exports {
                symbol_table
                    .try_insert(*import_key, import_val.clone())
                    .map_err(|_| {
                        Error::new(
                            format!(
                                "Import contains identifier '{}', which already exists in this scope",
                                import_key.fg(ATTENTION_COLOR)
                            ),
                            import.specifier.span,
                        )
                        .with_note(format!(
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
        match self.eval(symbol_table)? {
            ExpressionResult::String(string) => Ok(((string.len() as f32) / 4.0).ceil() as u32),
            ExpressionResult::Number(_number) => Ok(1),
            _ => Err(Error::new(
                format!(
                    "expected a {} or {}",
                    "string".fg(ATTENTION_COLOR),
                    "number".fg(ATTENTION_COLOR)
                ),
                self.span,
            )),
        }
    }
}

impl Operation {
    pub fn num_words(&self) -> Result<u32, Error> {
        Ok(1)
    }
}
