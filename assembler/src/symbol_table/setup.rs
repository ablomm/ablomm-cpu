use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ariadne::Fmt;
use internment::Intern;

use crate::{
    ast::{Block, Expression, File, Import, ImportSpecifier, Operation, Statement},
    error::{ATTENTION_COLOR, RecoveredError, RecoveredResult, SpannedError},
    expression::{
        EvalReturn,
        expression_result::{ExpressionResult, Number},
    },
    span::Spanned,
    src::{self, Src},
    symbol_table::{self, Symbol},
};

use super::{STEntry, SymbolTable};

type ExportMap = HashMap<Intern<String>, symbol_table::Value>;

pub fn init_symbol_tables(file_queue: &mut [Spanned<File>]) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();

    let mut file_exports_map = HashMap::new();
    for file in file_queue.iter() {
        let exports = match add_symbols(&file.span_to(&file.block)) {
            Ok(exports) => exports,
            Err(RecoveredError(exports, mut symbol_errors)) => {
                errors.append(&mut symbol_errors);
                exports
            }
        };

        file_exports_map.insert(file.src, exports);
    }

    for file in file_queue.iter() {
        match add_imports(&file.src, &file.span_to(&file.block), &file_exports_map) {
            Ok(_) => (),
            Err(mut import_errors) => errors.append(&mut import_errors),
        }
    }

    let mut address_accumulator = 0;
    for file in file_queue.iter() {
        address_accumulator = match set_labels(address_accumulator, &file.span_to(&file.block)) {
            Ok(address) => address,
            Err(RecoveredError(address, mut label_errors)) => {
                errors.append(&mut label_errors);
                address
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// generates symbol table for block and sub_blocks (excluding imports), returns exported symbols
fn add_symbols(block: &Spanned<&Block>) -> RecoveredResult<ExportMap> {
    let mut exports = HashMap::new();
    let mut errors = Vec::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                let result = label.identifier.span_to(ExpressionResult::Number(None));

                match block.symbol_table.borrow_mut().try_insert(
                    label.identifier.val,
                    STEntry {
                        symbol: Rc::new(RefCell::new(Symbol {
                            result: Some(result),
                            expression: None,
                            symbol_table: Rc::clone(&block.symbol_table),
                        })),
                        key_span: label.identifier.span,
                        import_span: None,
                        export_span: None,
                    },
                ) {
                    Ok(_) => (),
                    Err(insert_error) => errors.push(insert_error),
                }

                if label.export {
                    match export(
                        &label.identifier,
                        &block.symbol_table.borrow(),
                        &mut exports,
                    ) {
                        Ok(_) => (),
                        Err(mut export_errors) => errors.append(&mut export_errors),
                    }
                }
            }

            Statement::Assignment(assignment) => {
                match block.symbol_table.borrow_mut().try_insert(
                    assignment.identifier.val,
                    STEntry {
                        symbol: Rc::new(RefCell::new(Symbol {
                            result: None,
                            expression: Some(assignment.expression.clone()),
                            symbol_table: Rc::clone(&block.symbol_table),
                        })),
                        key_span: assignment.identifier.span,
                        import_span: None,
                        export_span: None,
                    },
                ) {
                    Ok(_) => (),
                    Err(insert_error) => errors.push(insert_error),
                }

                if assignment.export {
                    match export(
                        &assignment.identifier,
                        &block.symbol_table.borrow(),
                        &mut exports,
                    ) {
                        Ok(_) => (),
                        Err(mut export_errors) => errors.append(&mut export_errors),
                    }
                }
            }

            Statement::Export(identifiers) => {
                for identifier in identifiers {
                    match export(identifier, &block.symbol_table.borrow(), &mut exports) {
                        Ok(_) => (),
                        Err(mut export_errors) => errors.append(&mut export_errors),
                    }
                }
            }

            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));

                let sub_exports = match add_symbols(&statement.span_to(sub_block)) {
                    Ok(sub_exports) => sub_exports,
                    Err(RecoveredError(sub_exports, mut sub_errors)) => {
                        errors.append(&mut sub_errors);
                        sub_exports
                    }
                };

                for (key, val) in sub_exports {
                    match block.symbol_table.borrow_mut().try_insert(key, val) {
                        Ok(_) => (),
                        Err(insert_error) => errors.push(insert_error),
                    }
                }
            }
            _ => (),
        }
    }

    if errors.is_empty() {
        Ok(exports)
    } else {
        Err(RecoveredError(exports, errors))
    }
}

// add every imported identifier to each symbol table whose scope imported it
fn add_imports(
    src: &Intern<Src>,
    block: &Spanned<&Block>,
    file_exports_map: &HashMap<Intern<Src>, ExportMap>,
) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Import(import_val) => {
                let import_src = match src::get_import_src(src, import_val) {
                    Ok(import_src) => import_src,

                    // we should have already inserted the error previously when generating file_queue
                    // TODO: save the import src in the import struct so we don't have to recompute it,
                    // which allows deleting or modifing the files during compilation with no issues
                    // also would allow continue on None (which IMO is more intuitive then on Err)
                    Err(_) => continue,
                };

                // the import files' exports
                let exports = file_exports_map.get(&import_src).unwrap_or_else(||
                    // add_symbols() should have already created it
                    panic!(
                        "Attempted to import '{}' when the exporter's symbol table has not been filled",
                        import_src
                    ));

                match import(import_val, &mut block.symbol_table.borrow_mut(), exports) {
                    Ok(_) => (),
                    Err(import_error) => errors.push(import_error),
                }
            }

            Statement::Block(sub_block) => {
                match add_imports(src, &statement.span_to(sub_block), file_exports_map) {
                    Ok(_) => (),
                    Err(mut sub_errors) => errors.append(&mut sub_errors),
                }
            }
            _ => (),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// calculates label addresses
fn set_labels(start_address: u32, block: &Spanned<&Block>) -> RecoveredResult<u32> {
    let mut address = start_address;
    let mut errors = Vec::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                let result = label
                    .identifier
                    .span_to(ExpressionResult::Number(Some(Number(address))));

                let symbol_table = block.symbol_table.borrow_mut();

                let entry = symbol_table
                    .get(&label.identifier.as_ref())
                    .unwrap_or_else(|| {
                        panic!("Label '{}' type did not exist in symbol table when inserting final value", label.identifier.val)
                    });

                entry.symbol.borrow_mut().result = Some(result);
            }

            Statement::Block(sub_block) => {
                match set_labels(address, &statement.span_to(sub_block)) {
                    Ok(_) => (),
                    Err(RecoveredError(_, mut sub_errors)) => errors.append(&mut sub_errors),
                }
            }
            _ => (),
        }

        address += match statement.as_ref().num_words(&block.symbol_table.borrow()) {
            Ok(length) => length,
            Err(error) => {
                errors.push(error);
                0
            }
        }
    }

    if errors.is_empty() {
        Ok(address)
    } else {
        Err(RecoveredError(address, errors))
    }
}

fn export(
    identifier: &Spanned<Intern<String>>,
    symbol_table: &SymbolTable,
    exports: &mut ExportMap,
) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();
    if let Some(entry) = exports.get(&identifier.val) {
        errors.push(
            SpannedError::new(identifier.span, "Identifier already exported")
                .with_label_span(
                    entry.export_span.unwrap_or_else(|| {
                        panic!(
                            "Exported identifier {} doesn't have export_span",
                            identifier.val
                        )
                    }),
                    "Exported first here",
                )
                .with_label("Exported again here")
                .with_help("Try removing one of these exports"),
        );
    }

    let mut export_entry = match symbol_table.try_get(&identifier.as_ref()) {
        Ok(export_entry) => export_entry,
        Err(error) => {
            errors.push(error);
            return Err(errors);
        }
    };

    export_entry.export_span = Some(identifier.span);
    exports.insert(identifier.val, export_entry);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn import(
    import: &Import,
    symbol_table: &mut SymbolTable,
    exports: &ExportMap,
) -> Result<(), SpannedError> {
    match &import.specifier.val {
        // selective import (i.e. import <ident> [as <ident>[, ...]] from <file>
        ImportSpecifier::Named(named_imports) => {
            for named_import in named_imports {
                let import_entry = exports.get(&named_import.identifier).ok_or(
                    SpannedError::new(named_import.identifier.span, "Identifier not found")
                        .with_label(format!(
                            "Identifier is not exported in '{}'",
                            import.file.val.as_str().fg(ATTENTION_COLOR)
                        )),
                )?;

                // if the import is aliased, then treat it as a new definition, in regards to
                // errors, since the names are different (i.e., it is defined at the alias
                // identifier instead of the definition inside the import)
                let import_span = if named_import.alias.is_some() {
                    None
                } else {
                    Some(import.specifier.span)
                };

                let original_definition =
                    import_entry.key_span.spanned(named_import.identifier.val);
                let import_key = named_import.alias.as_ref().unwrap_or(&original_definition);

                symbol_table
                    .try_insert(
                        import_key.val,
                        STEntry {
                            symbol: Rc::clone(&import_entry.symbol),
                            key_span: import_entry.key_span,
                            import_span,
                            export_span: import_entry.export_span,
                        },
                    )
                    .map_err(|error| {
                        error.with_help(format!(
                            "Try aliasing the import by adding {}",
                            "as <new_name>".fg(ATTENTION_COLOR)
                        ))
                    })?;
            }
        }

        // unselective import (i.e. import * from <file>
        ImportSpecifier::Blob => {
            for (import_key, import_entry) in exports {
                symbol_table
                    .try_insert(
                        *import_key,
                        STEntry {
                            symbol: Rc::clone(&import_entry.symbol),
                            key_span: import_entry.key_span,
                            import_span: Some(import.specifier.span),
                            export_span: import_entry.export_span,
                        },
                    )
                    .map_err(|error| {
                        error.with_help(format!(
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

impl Block {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, SpannedError> {
        let mut num_words = 0;
        for statement in &self.statements {
            num_words += statement.as_ref().num_words(symbol_table)?;
        }

        Ok(num_words)
    }
}

impl Spanned<&Statement> {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, SpannedError> {
        match self.val {
            Statement::GenLiteral(literal) => self.span_to(literal).num_words(symbol_table),
            Statement::Block(block) => block.num_words(&block.symbol_table.borrow()),
            Statement::Operation(operation) => operation.num_words(),
            _ => Ok(0),
        }
    }
}

impl Spanned<&Expression> {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, SpannedError> {
        let EvalReturn {
            result,
            waiting_map,
        } = self.eval(symbol_table)?;

        match result {
            ExpressionResult::Number(_number) => Ok(1),
            ExpressionResult::String(string) => {
                let string = string.ok_or_else(|| {
                    let mut error = SpannedError::new(self.span, "Unknown value of expression").with_label(
                        "Expression needs to be determined, but is not",
                    );
                    for span in waiting_map.values() {
                        error = error.with_label_span(*span, "This value is undetermined")
                    }

                    error.with_note(
                        "This is ultimately caused because the expression is dependent on a future address (label), but the value of the expression would effect that address (label)",
                    ).with_note(
                        "For more info, see https://github.com/ablomm/ablomm-cpu/blob/main/docs/assembler/errors.md#unknown-value-of-expression",
                    )
                })?;

                Ok(((string.len() as f32) / 4.0).ceil() as u32)
            }
            _ => Err(SpannedError::incorrect_value(
                self.span,
                "type",
                vec!["number", "string"],
                Some(result),
            )),
        }
    }
}

impl Operation {
    pub fn num_words(&self) -> Result<u32, SpannedError> {
        Ok(1)
    }
}
