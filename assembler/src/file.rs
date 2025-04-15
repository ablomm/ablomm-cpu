use chumsky::prelude::*;
use std::{collections::HashMap, fs};

use indexmap::IndexMap;
use internment::Intern;

use crate::{
    ast::{Block, File, Import, Statement},
    error::SpannedError,
    parser,
    span::{Span, Spanned},
    src::{self, Src},
};

// takes in a canonical file path, address, and cache and returns a queue of the order in which to
// generate the symbol tables / exports in order to satisfy import dependencies (i.e. post order)
pub fn generate_file_queue(
    src: &Spanned<Intern<Src>>,
    cache: &mut HashMap<Intern<Src>, String>,
    import_map: &mut IndexMap<Intern<Src>, Span>, // to detect cycles, IndexSet to preserve insert order; useful for errors
) -> Result<Vec<Spanned<File>>, Vec<SpannedError>> {
    import_map.insert(src.val, src.span);

    let mut file_queue = Vec::new();

    // parse itself
    let file = parse_file(src, cache)?;

    // after finding correct addresses
    for import in file.block.get_imports() {
        let import_src = match src::get_import_src(src, import) {
            Ok(import_src) => import.file.span_to(import_src),
            Err(error) => {
                return Err(vec![SpannedError::new(
                    import.file.span,
                    "Error finding file",
                )
                .with_label(error.to_string())])
            }
        };

        if import_map.contains_key(&import_src.val) {
            // circular dependency
            let mut error = SpannedError::new(import_src.span, "Circular dependency");

            // skipping one because the first one is always the root file, which has no
            // corresponding import statement
            for (_back_src, back_span) in import_map.iter().skip(1) {
                error = error.with_label_span(*back_span, "Imported here");
            }
            error = error
                .with_label("This completes the circle, causing a circular dependency")
                .with_help("Try removing one of these imports");

            return Err(vec![error]);
        }

        if cache.contains_key(&import_src.val) {
            // we already did this import, but in another branch (so not a circular dependency), so skip it
            continue;
        }

        file_queue.append(&mut generate_file_queue(&import_src, cache, import_map)?);
    }

    // only add itself after imports are added (post order)
    file_queue.push(file);

    import_map.shift_remove(&src.val);
    Ok(file_queue)
}

// takes a src, reads it, and parses the contents
fn parse_file(
    src: &Spanned<Intern<Src>>,
    cache: &mut HashMap<Intern<Src>, String>,
) -> Result<Spanned<File>, Vec<SpannedError>> {
    // need to do a match here because map_err causes the borrow checker to think that cache is
    // moved into the map_err closure
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
        start_address: None,
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
