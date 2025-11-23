use std::collections::HashMap;

use internment::Intern;

use crate::{
    SpannedError,
    ast::File,
    error::RecoveredError,
    span::Spanned,
    symbol_table::{self},
};

mod imports;
mod labels;
mod symbols;

type ExportMap = HashMap<Intern<String>, symbol_table::Value>;

pub fn init_symbol_tables(file_queue: &mut [Spanned<File>]) -> Result<(), Vec<SpannedError>> {
    let mut errors = Vec::new();

    let mut file_exports_map = HashMap::new();
    for file in file_queue.iter_mut() {
        let exports = match symbols::add_symbols(file.span.spanned(&mut file.block)) {
            Ok(exports) => exports,
            Err(RecoveredError(exports, mut symbol_errors)) => {
                errors.append(&mut symbol_errors);
                exports
            }
        };

        file_exports_map.insert(file.src, exports);
    }

    for file in file_queue.iter_mut() {
        match imports::add_imports(
            file.src,
            file.span.spanned(&mut file.block),
            &file_exports_map,
        ) {
            Ok(_) => (),
            Err(mut import_errors) => errors.append(&mut import_errors),
        }
    }

    let mut address_accumulator = 0;
    for file in file_queue.iter_mut() {
        address_accumulator =
            match labels::set_labels(address_accumulator, file.span.spanned(&mut file.block)) {
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
