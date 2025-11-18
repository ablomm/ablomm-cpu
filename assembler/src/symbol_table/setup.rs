use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ariadne::Fmt;
use internment::Intern;

use crate::{
    ast::{Block, Expression, File, Import, ImportSpecifier, Operation, Statement},
    error::{ATTENTION_COLOR, SpannedError},
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

pub fn init_symbol_tables(file_queue: &mut [Spanned<File>]) -> Result<(), SpannedError> {
    let mut file_exports_map = HashMap::new();
    for file in file_queue.iter() {
        let exports = infer_types(&file.span_to(&file.block))?;
        file_exports_map.insert(file.src, exports);
    }

    for file in file_queue.iter() {
        add_imports(&file.src, &file.span_to(&file.block), &file_exports_map)?;
    }

    let mut address_accumulator = 0;
    for file in file_queue.iter() {
        address_accumulator = fill_labels(address_accumulator, &file.span_to(&file.block))?;
    }

    Ok(())
}

// generates symbol table for block and sub_blocks, returns exported symbols with inferred types
fn infer_types(block: &Spanned<&Block>) -> Result<ExportMap, SpannedError> {
    let mut exports = HashMap::new();

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                let result = label.identifier.span_to(ExpressionResult::Number(None));

                block.symbol_table.borrow_mut().try_insert(
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
                block.symbol_table.borrow_mut().try_insert(
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

            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));

                let sub_exports = infer_types(&statement.span_to(sub_block))?;

                for (key, val) in sub_exports {
                    block.symbol_table.borrow_mut().try_insert(key, val)?;
                }
            }
            _ => (),
        }
    }

    Ok(exports)
}

// add every imported identifier to each symbol table whose scope imported it
fn add_imports(
    src: &Intern<Src>,
    block: &Spanned<&Block>,
    file_exports_map: &HashMap<Intern<Src>, ExportMap>,
) -> Result<(), SpannedError> {
    for statement in &block.statements {
        match &statement.val {
            Statement::Import(import_val) => {
                let import_src = src::get_import_src(src, import_val).unwrap_or_else(|error| {
                    // infer_types() should have already created it
                    panic!(
                        "Could not find import '{}' in file '{}': {}",
                        import_val.file.val, src, error
                    )
                });

                // the import files' exports
                let exports = file_exports_map.get(&import_src).unwrap_or_else(||
                    // infer_types() should have already created it
                    panic!(
                        "Attempted to import '{}' when the exporter's symbol table has not been filled",
                        import_src
                    ));

                import(import_val, &mut block.symbol_table.borrow_mut(), exports)?
            }

            Statement::Block(sub_block) => {
                add_imports(src, &statement.span_to(sub_block), file_exports_map)?;
            }
            _ => (),
        }
    }

    Ok(())
}

// calculat label addresses
fn fill_labels(start_address: u32, block: &Spanned<&Block>) -> Result<u32, SpannedError> {
    let mut address = start_address;
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
                        panic!("Label {} type did not exist in symbol table when inserting final value", label.identifier.val)
                    });

                entry.symbol.borrow_mut().result = Some(result);
            }

            Statement::Block(sub_block) => {
                fill_labels(address, &statement.span_to(sub_block))?;
            }
            _ => (),
        }

        address += statement.as_ref().num_words(&block.symbol_table.borrow())?;
    }

    Ok(address)
}

fn export(
    identifier: &Spanned<Intern<String>>,
    symbol_table: &SymbolTable,
    exports: &mut ExportMap,
) -> Result<(), SpannedError> {
    if let Some(entry) = exports.get(&identifier.val) {
        return Err(
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
                .with_note("Try removing one of these exports"),
        );
    }

    let mut export_entry = symbol_table.try_get(&identifier.as_ref())?;
    export_entry.export_span = Some(identifier.span);
    exports.insert(identifier.val, export_entry);

    Ok(())
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
                // errors, since the namesare different (i.e., it is defined at the alias
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
