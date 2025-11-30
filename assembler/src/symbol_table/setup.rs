use crate::{
    ast::Ast,
    error::{Error, RecoveredError},
};

mod imports;
mod labels;
mod symbols;

impl Ast {
    pub fn init_symbol_tables(&mut self) -> Result<(), Vec<Error>> {
        let mut errors = Vec::new();

        let file_exports_map = match self.add_symbols() {
            Ok(file_exports_map) => file_exports_map,
            Err(RecoveredError(file_exports_map, mut symbol_errors)) => {
                errors.append(&mut symbol_errors);
                file_exports_map
            }
        };

        match self.add_imports(&file_exports_map) {
            Ok(_) => (),
            Err(mut export_errors) => errors.append(&mut export_errors),
        }

        match self.set_labels() {
            Ok(_) => (),
            Err(mut import_errors) => errors.append(&mut import_errors),
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
