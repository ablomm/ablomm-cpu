use chumsky::prelude::*;
use std::{collections::HashMap, fs};

use internment::Intern;

use crate::{
    ast::{Block, File, Import, Statement},
    error::SpannedError,
    parser,
    span::Spanned,
    src::{self, Src},
};

// takes in a canonical file path, address, and cache and returns a queue of the order in which to
// generate the machine code
pub fn generate_file_queue(
    src: &Spanned<Intern<Src>>,
    cache: &mut HashMap<Intern<Src>, String>,
) -> Result<Vec<Spanned<File>>, Vec<SpannedError>> {
    let mut file_queue = Vec::new();

    let file = parse_file(src, cache)?;

    for import in file.block.get_imports() {
        let import_src = match src::get_import_src(src, import) {
            Ok(import_src) => import.file.span_to(import_src),
            Err(error) => {
                return Err(vec![
                    SpannedError::new(import.file.span, "Error finding file")
                        .with_label(error.to_string()),
                ]);
            }
        };

        if cache.contains_key(&import_src.val) {
            // we already did this import
            continue;
        }

        file_queue.append(&mut generate_file_queue(&import_src, cache)?);
    }

    // only add itself after imports are added to satisfy borrow checker
    file_queue.push(file);

    // depth-first
    Ok(file_queue.into_iter().rev().collect())
}

// takes a src, reads it, and parses the contents
fn parse_file(
    src: &Spanned<Intern<Src>>,
    cache: &mut HashMap<Intern<Src>, String>,
) -> Result<Spanned<File>, Vec<SpannedError>> {
    let assembly_code = fs::read_to_string(src.as_path()).map_err(|error| {
        vec![SpannedError::new(src.span, "Error reading file").with_label(error.to_string())]
    })?;

    // need to insert before parsing so it can show parsing errors
    cache.insert(src.val, assembly_code);
    let assembly_code = cache.get(&src.val).unwrap(); // unwrap safe because we just inserted

    let block = parser::file_block_parser()
        .parse(assembly_code.with_context(src.val))
        // workaround because chumsky does not work well with Error:
        // convert to Error after parsing rather than use Error during parsing
        .into_result()
        .map_err(|errors| {
            errors
                .into_iter()
                .map(|error| error.into())
                .collect::<Vec<SpannedError>>()
        })?;
    let file = File {
        src: src.val,
        block: block.val,
    };

    Ok(block.span.spanned(file))
}

impl Block {
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
