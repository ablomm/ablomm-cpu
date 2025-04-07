use std::{collections::HashMap, rc::Rc};

use ariadne::Fmt;
use internment::Intern;

use crate::{
    ast::{Block, Expression, File, Import, ImportSpecifier, Operation, Statement},
    error::{SpannedError, ATTENTION_COLOR},
    expression::{
        expression_result::{ExpressionResult, Number},
        EvalReturn,
    },
    span::Spanned,
    src::{self, Src},
};

use super::{STEntry, SymbolTable};

pub fn fill_symbol_tables(file_queue: &Vec<Spanned<File>>) -> Result<(), SpannedError> {
    let mut file_exports_map = HashMap::new();
    for file in file_queue {
        // can't do map_err because of borrow checker
        let exports = fill_symbol_table(
            &file.src,
            &file.span_to(&file.block),
            file.start_address,
            &file_exports_map,
        )?;

        file_exports_map.insert(file.src, exports);
    }
    Ok(())
}

// generates symbol table for block and sub_blocks, returns exported symbols
fn fill_symbol_table(
    src: &Intern<Src>,
    block: &Spanned<&Block>,
    // if start_address is None, it will simply fill the symbol_table to the best of it's ability
    // without any addresses
    start_address: Option<u32>,
    file_exports_map: &HashMap<Intern<Src>, HashMap<Intern<String>, STEntry>>,
) -> Result<HashMap<Intern<String>, STEntry>, SpannedError> {
    block.symbol_table.borrow_mut().set_entries_updatable();

    let mut address = start_address;

    let mut exports = HashMap::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                let result = label
                    .identifier
                    .span_to(ExpressionResult::Number(address.map(Number)));

                block.symbol_table.borrow_mut().try_insert(
                    label.identifier.val,
                    STEntry::new(result, label.identifier.span),
                )?;

                if label.export {
                    export(
                        &label.identifier,
                        &block.symbol_table.borrow(),
                        &mut exports,
                    )?;
                }
            }

            Statement::Assignment(assignment) => {
                block.symbol_table.borrow_mut().try_insert_expr(
                    assignment.identifier.val,
                    &assignment.expression.as_ref(),
                    assignment.identifier.span,
                    None,
                    None,
                    false,
                )?;

                if assignment.export {
                    export(
                        &assignment.identifier,
                        &block.symbol_table.borrow(),
                        &mut exports,
                    )?;
                }
            }

            Statement::Export(identifiers) => {
                for identifier in identifiers {
                    export(identifier, &block.symbol_table.borrow(), &mut exports)?;
                }
            }

            Statement::Import(import_val) => {
                let import_src = src::get_import_src(src, import_val).unwrap_or_else(|error| {
                    // should not occur because should already have gotten this import in
                    // generate_file_queue()
                    panic!(
                        "Could not find import '{}' in file '{}': {}",
                        import_val.file.val, src, error
                    )
                });

                // the import files' exports
                let exports = file_exports_map.get(&import_src).unwrap_or_else(||
                    // should not occur because the order should ensure all dependencies are parsed
                    panic!(
                        "Attempted to import '{}' when the exporter's symbol table has not been filled",
                        import_src
                    ));

                import(import_val, &mut block.symbol_table.borrow_mut(), exports)?
            }

            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));

                let sub_exports = fill_symbol_table(
                    src,
                    &statement.span_to(sub_block),
                    address,
                    file_exports_map,
                )?;

                for (key, val) in sub_exports {
                    block.symbol_table.borrow_mut().try_insert(key, val)?;
                }
            }
            _ => (),
        }

        // technically we could count the lines in the loop above, but this is a bit more readable
        // even though it requires another pass
        if let Some(mut address_val) = address {
            address_val += statement.as_ref().num_words(&block.symbol_table.borrow())?;
            address = Some(address_val);
        }
    }

    Ok(exports)
}

// needed because some future imports symbols depend on the size of the importer's length
// (yeah, I know; it's confusing)
// assume file_queue is in post_order
pub fn calculate_addresses(file_queue: &mut [Spanned<File>]) -> Result<(), SpannedError> {
    let mut address = 0;
    for file in file_queue.iter_mut().rev() {
        file.block.symbol_table.borrow_mut().set_entries_updatable();

        file.val.start_address = Some(address);
        for statment in &file.block.statements {
            match &statment.val {
                // we now are able to calculate labels, which gives more values that can be used in
                // num_words()
                Statement::Label(label) => {
                    let result = label
                        .identifier
                        .span_to(ExpressionResult::Number(Some(Number(address))));

                    file.block.symbol_table.borrow_mut().try_insert(
                        label.identifier.val,
                        STEntry::new(result, label.identifier.span),
                    )?;
                }

                // now that the labels are known, we need to re-evaluate assignments, which may
                // depend on the label values
                // don't worry about exports, as the file_queue is in post_order which guarantees
                // all importers have already been parsed, so no one will use the new exports
                Statement::Assignment(assignment) => {
                    file.block.symbol_table.borrow_mut().try_insert_expr(
                        assignment.identifier.val,
                        &assignment.expression.as_ref(),
                        assignment.identifier.span,
                        None,
                        None,
                        false,
                    )?;
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
) -> Result<(), SpannedError> {
    if let Some(entry) = exports.get(&identifier.val) {
        return Err(
            SpannedError::new(identifier.span, "Identifier already exported")
                .with_label_span(
                    entry
                        .export_span
                        .expect("Exported identifier doesn't have export_span"),
                    "Exported first here",
                )
                .with_label("Exported again here")
                .with_note("Try removing one of these exports"),
        );
    }

    let mut export_val = symbol_table.try_get(&identifier.as_ref())?;
    export_val.export_span = Some(identifier.span);
    exports.insert(identifier.val, export_val);

    Ok(())
}

fn import(
    import: &Import,
    symbol_table: &mut SymbolTable,
    exports: &HashMap<Intern<String>, STEntry>,
) -> Result<(), SpannedError> {
    match &import.specifier.val {
        // selective import (i.e. import <ident> [as <ident>[, ...]] from <file>
        ImportSpecifier::Named(named_imports) => {
            for named_import in named_imports {
                let import_val = exports.get(&named_import.identifier).ok_or(
                    SpannedError::new(named_import.identifier.span, "Identifier not found")
                        .with_label(format!(
                            "Identifier is not exported in '{}'",
                            import.file.val.as_str().fg(ATTENTION_COLOR)
                        )),
                )?;

                let original_definition = import_val.key_span.spanned(named_import.identifier.val);
                let import_key = named_import.alias.as_ref().unwrap_or(&original_definition);

                // if the import is aliased, then treat it as a new definition, since the names
                // are different (i.e., it is defined at the alias identifier instead of the
                // definition inside the import)
                let import_span = if named_import.alias.is_some() {
                    None
                } else {
                    Some(import.specifier.span)
                };

                let st_entry = STEntry {
                    key_span: import_key.span,
                    import_span,
                    ..import_val.clone()
                };

                symbol_table
                    .try_insert(import_key.val, st_entry)
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
            for (import_key, import_val) in exports {
                let st_entry = STEntry {
                    import_span: Some(import.specifier.span),
                    ..import_val.clone()
                };

                symbol_table
                    .try_insert(*import_key, st_entry)
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
            Statement::Block(block) => block.num_words(symbol_table),
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
