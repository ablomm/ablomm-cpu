use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ariadne::Fmt;
use internment::Intern;

use crate::{
    ast::{Block, Import, ImportSpecifier, Statement},
    error::{ATTENTION_COLOR, SpannedError},
    span::Spanned,
    src::{self, Src},
    symbol_table::{STEntry, SymbolTable, setup::ExportMap},
};

// add every imported identifier to each symbol table whose scope imported it
pub fn add_imports(
    src: Intern<Src>,
    mut block: Spanned<&mut Block>,
    file_exports_map: &HashMap<Intern<Src>, ExportMap>,
) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();

    // to satisfy borrow checker
    let symbol_table = Rc::clone(&block.symbol_table);

    // filter out statements that cause errors so that an erroneous statement doesn't cascade errors
    block.statements.retain_mut(|statement| {
        match statement_add_imports(src, statement.as_mut_ref(), &symbol_table, file_exports_map) {
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

fn statement_add_imports(
    src: Intern<Src>,
    statement: Spanned<&mut Statement>,
    symbol_table: &Rc<RefCell<SymbolTable>>,
    file_exports_map: &HashMap<Intern<Src>, ExportMap>,
) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();

    match statement.val {
        Statement::Import(import_val) => {
            let import_src = src::get_import_src(src, import_val).unwrap_or_else(|_| {
                // should not occur because generate_file_queue() should have filtered out any
                // imports that are not valid
                // TODO: save Src from generate_file_queue() so we don't have to compute it again,
                // and also allows for deleting the file during compilation and not failing
                panic!(
                    "Attempted to get Src of '{}' (from '{}'), but found errors",
                    import_val.file.val, src
                )
            });

            // the import files' exports
            let exports = file_exports_map.get(&import_src).unwrap_or_else(||
                    // add_symbols() should have already created it
                    panic!(
                        "Attempted to import '{}' when the exporter's symbol table has not been filled",
                        import_src
                    ));

            match import(import_val, &mut symbol_table.borrow_mut(), exports) {
                Ok(_) => (),
                Err(import_error) => errors.push(import_error),
            }
        }

        Statement::Block(sub_block) => {
            match add_imports(src, statement.span.spanned(sub_block), file_exports_map) {
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
