use std::{cell::RefCell, rc::Rc};

use ariadne::Fmt;

use crate::{
    ast::{Ast, Block, File, Import, ImportSpecifier, Statement},
    error::{ATTENTION_COLOR, SpannedError},
    span::Spanned,
    symbol_table::{
        STEntry, SymbolTable,
        setup::symbols::{ExportMap, FileExportMap},
    },
};

impl Ast {
    // adds imports to importers' symbol table
    pub fn add_imports(
        &mut self,
        file_exports_map: &FileExportMap,
    ) -> Result<(), Vec<SpannedError>> {
        let mut errors = Vec::new();

        for file in self.files.iter_mut() {
            match file.as_mut_ref().add_imports(file_exports_map) {
                Ok(_) => (),
                Err(mut import_errors) => errors.append(&mut import_errors),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Spanned<&mut File> {
    fn add_imports(&mut self, file_exports_map: &FileExportMap) -> Result<(), Vec<SpannedError>> {
        self.span
            .spanned(&mut self.block)
            .add_imports(file_exports_map)
    }
}

impl Spanned<&mut Block> {
    fn add_imports(&mut self, file_exports_map: &FileExportMap) -> Result<(), Vec<SpannedError>> {
        let mut errors = Vec::new();

        // to satisfy borrow checker
        let symbol_table = Rc::clone(&self.symbol_table);

        // filter out statements that cause errors so that an erroneous statement doesn't cascade errors
        self.statements.retain_mut(|statement| {
            match statement
                .as_mut_ref()
                .add_imports(&symbol_table, file_exports_map)
            {
                Ok(_) => true,
                Err(mut statement_errors) => {
                    errors.append(&mut statement_errors);
                    false
                }
            }
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Spanned<&mut Statement> {
    fn add_imports(
        &mut self,
        symbol_table: &Rc<RefCell<SymbolTable>>,
        file_exports_map: &FileExportMap,
    ) -> Result<(), Vec<SpannedError>> {
        let mut errors = Vec::new();

        match self.val {
            Statement::Import(import) => {
                // the import files' exports
                let exports = file_exports_map.get(&import.src).unwrap_or_else(||
                    // add_symbols() should have already created it
                    panic!(
                        "Attempted to import '{}' when the exporter's symbol table has not been filled",
                        import.src.val
                    ));

                match symbol_table.borrow_mut().import(import, exports) {
                    Ok(_) => (),
                    Err(import_error) => errors.push(import_error),
                }
            }

            Statement::Block(sub_block) => {
                match self.span.spanned(sub_block).add_imports(file_exports_map) {
                    Ok(_) => (),
                    Err(mut sub_errors) => errors.append(&mut sub_errors),
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

impl SymbolTable {
    fn import(&mut self, import: &Import, exports: &ExportMap) -> Result<(), SpannedError> {
        match &import.specifier.val {
            // selective import (i.e. import <ident> [as <ident>[, ...]] from <file>
            ImportSpecifier::Named(named_imports) => {
                for named_import in named_imports {
                    let import_entry = exports.get(&named_import.identifier).ok_or(
                        SpannedError::new(named_import.identifier.span, "Identifier not found")
                            .with_label(format!(
                                "Identifier is not exported in '{}'",
                                import.src.val.fg(ATTENTION_COLOR)
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

                    self.try_insert(
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
                    self.try_insert(
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
}
