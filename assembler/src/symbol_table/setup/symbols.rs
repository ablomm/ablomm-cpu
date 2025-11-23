use std::{cell::RefCell, rc::Rc};

use internment::Intern;

use crate::{
    ast::{Block, Statement},
    error::{RecoveredError, RecoveredResult, SpannedError},
    expression::expression_result::ExpressionResult,
    span::Spanned,
    symbol_table::{STEntry, Symbol, SymbolTable, setup::ExportMap},
};

// generates symbol table for block and sub_blocks (excluding imports), returns exported symbols
pub fn add_symbols(mut block: Spanned<&mut Block>) -> RecoveredResult<ExportMap> {
    let mut exports = ExportMap::new();
    let mut errors = Vec::new();

    // to satisfy borrow checker
    let symbol_table = Rc::clone(&block.symbol_table);

    // filter out statements that cause errors so that an erroneous statement doesn't cascade errors
    block.statements.retain_mut(|statement| {
        match statement_add_symbols(statement.as_mut_ref(), &symbol_table, &mut exports) {
            Ok(_) => true,
            Err(mut statement_errors) => {
                errors.append(&mut statement_errors);
                false
            }
        }
    });

    if errors.is_empty() {
        Ok(exports)
    } else {
        Err(RecoveredError(exports, errors))
    }
}

fn statement_add_symbols(
    statement: Spanned<&mut Statement>,
    symbol_table: &Rc<RefCell<SymbolTable>>,
    exports: &mut ExportMap,
) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();

    match statement.val {
        Statement::Label(label) => {
            let result = label.identifier.span_to(ExpressionResult::Number(None));
            let st_entry = STEntry {
                symbol: Rc::new(RefCell::new(Symbol {
                    result: Some(result),
                    expression: None,
                    symbol_table: Rc::clone(symbol_table),
                })),
                key_span: label.identifier.span,
                import_span: None,
                export_span: None,
            };

            match symbol_table
                .borrow_mut()
                .try_insert(label.identifier.val, st_entry.clone())
            {
                Ok(_) => (),
                Err(insert_error) => errors.push(insert_error),
            }

            if label.export {
                match add_export(label.identifier, st_entry, exports) {
                    Ok(_) => (),
                    Err(export_insert_error) => errors.push(export_insert_error),
                }
            }
        }

        Statement::Assignment(assignment) => {
            let st_entry = STEntry {
                symbol: Rc::new(RefCell::new(Symbol {
                    result: None,
                    expression: Some(assignment.expression.clone()),
                    symbol_table: Rc::clone(symbol_table),
                })),
                key_span: assignment.identifier.span,
                import_span: None,
                export_span: None,
            };

            match symbol_table
                .borrow_mut()
                .try_insert(assignment.identifier.val, st_entry.clone())
            {
                Ok(_) => (),
                Err(insert_error) => errors.push(insert_error),
            }

            if assignment.export {
                match add_export(assignment.identifier, st_entry, exports) {
                    Ok(_) => (),
                    Err(export_insert_error) => errors.push(export_insert_error),
                }
            }
        }

        Statement::Export(identifiers) => {
            for identifier in identifiers {
                let entry = symbol_table
                    .borrow()
                    .try_get(&identifier.as_ref())
                    .map_err(|error| vec![error])?;

                match add_export(*identifier, entry, exports) {
                    Ok(_) => (),
                    Err(export_errors) => errors.push(export_errors),
                }
            }
        }

        Statement::Block(sub_block) => {
            sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(symbol_table));

            let sub_exports = match add_symbols(statement.span.spanned(sub_block)) {
                Ok(sub_exports) => sub_exports,
                Err(RecoveredError(sub_exports, mut sub_errors)) => {
                    errors.append(&mut sub_errors);
                    sub_exports
                }
            };

            for (key, val) in sub_exports {
                match symbol_table.borrow_mut().try_insert(key, val) {
                    Ok(_) => (),
                    Err(insert_error) => errors.push(insert_error),
                }
            }
        }
        _ => (),
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn add_export(
    identifier: Spanned<Intern<String>>,
    mut entry: STEntry,
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
                .with_help("Try removing one of these exports"),
        );
    }

    entry.export_span = Some(identifier.span);

    exports.insert(identifier.val, entry);
    Ok(())
}
