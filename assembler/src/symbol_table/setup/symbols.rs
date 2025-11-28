use std::{cell::RefCell, collections::HashMap, rc::Rc};

use internment::Intern;

use crate::{
    ast::{Ast, Block, File, Statement},
    error::{RecoveredError, RecoveredResult, SpannedError},
    expression::expression_result::ExpressionResult,
    span::Spanned,
    src::Src,
    symbol_table::{self, STEntry, Symbol, SymbolTable, SymbolValue},
};

pub type ExportMap = HashMap<symbol_table::Key, symbol_table::Value>;
pub type FileExportMap = HashMap<Intern<Src>, ExportMap>;

impl Ast {
    // generates symbol table for block and sub_blocks (excluding imports), returns exported symbols per file
    pub fn add_symbols(&mut self) -> RecoveredResult<FileExportMap> {
        let mut file_exports_map = FileExportMap::new();
        let mut errors = Vec::new();

        for file in self.files.iter_mut() {
            let exports = match file.as_mut_ref().add_symbols() {
                Ok(exports) => exports,
                Err(RecoveredError(exports, mut symbol_errors)) => {
                    errors.append(&mut symbol_errors);
                    exports
                }
            };

            file_exports_map.insert(file.span.src, exports);
        }

        if errors.is_empty() {
            Ok(file_exports_map)
        } else {
            Err(RecoveredError(file_exports_map, errors))
        }
    }
}

impl Spanned<&mut File> {
    fn add_symbols(&mut self) -> RecoveredResult<ExportMap> {
        self.span.spanned(&mut self.block).add_symbols()
    }
}

impl Spanned<&mut Block> {
    fn add_symbols(&mut self) -> RecoveredResult<ExportMap> {
        let mut exports = ExportMap::new();
        let mut errors = Vec::new();

        // to satisfy borrow checker
        let symbol_table = Rc::clone(&self.symbol_table);

        // filter out statements that cause errors so that an erroneous statement doesn't cascade errors
        self.statements.retain_mut(|statement| {
            match statement
                .as_mut_ref()
                .add_symbols(&symbol_table, &mut exports)
            {
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
}

impl Spanned<&mut Statement> {
    fn add_symbols(
        &mut self,
        symbol_table: &Rc<RefCell<SymbolTable>>,
        exports: &mut ExportMap,
    ) -> Result<(), Vec<SpannedError>> {
        let mut errors = Vec::new();

        match self.val {
            Statement::Label(label) => {
                let result = label.identifier.span_to(ExpressionResult::Number(None));
                let st_entry = STEntry {
                    symbol: Rc::new(RefCell::new(Symbol {
                        value: result.span.spanned(SymbolValue::Result(result.val)),
                        symbol_table: Rc::downgrade(symbol_table),
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
                    match add_export(exports, label.identifier, st_entry) {
                        Ok(_) => (),
                        Err(export_insert_error) => errors.push(export_insert_error),
                    }
                }
            }

            Statement::Assignment(assignment) => {
                let st_entry = STEntry {
                    symbol: Rc::new(RefCell::new(Symbol {
                        value: assignment
                            .expression
                            .span_to(SymbolValue::Expression(assignment.expression.val.clone())),
                        symbol_table: Rc::downgrade(symbol_table),
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
                    match add_export(exports, assignment.identifier, st_entry) {
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

                    match add_export(exports, *identifier, entry) {
                        Ok(_) => (),
                        Err(export_errors) => errors.push(export_errors),
                    }
                }
            }

            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(symbol_table));

                let sub_exports = match self.span.spanned(sub_block).add_symbols() {
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
}

fn add_export(
    exports: &mut ExportMap,
    identifier: Spanned<Intern<String>>,
    mut entry: STEntry,
) -> Result<(), SpannedError> {
    if let Some(entry) = exports.get(&identifier.val) {
        return Err(
            SpannedError::new(identifier.span, "Identifier already exported")
                .with_label_span(
                    entry.export_span.unwrap_or_else(|| {
                        panic!(
                            "Exported identifier '{}' at {} doesn't have export_span",
                            identifier.val, identifier.span
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
