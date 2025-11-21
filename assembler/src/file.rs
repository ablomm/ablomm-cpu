use ariadne::Cache;
use chumsky::prelude::*;
use std::collections::HashSet;

use internment::Intern;

use crate::{
    SrcCache,
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
    cache: &mut SrcCache,
) -> Result<Vec<Spanned<File>>, Vec<SpannedError>> {
    generate_file_queue_with_src_map(src, cache, &mut HashSet::new())
}

fn generate_file_queue_with_src_map(
    src: &Spanned<Intern<Src>>,
    cache: &mut SrcCache,
    src_map: &mut HashSet<Intern<Src>>,
) -> Result<Vec<Spanned<File>>, Vec<SpannedError>> {
    src_map.insert(src.val);

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

        if src_map.contains(&import_src.val) {
            // we already did this import
            continue;
        }

        file_queue.append(&mut generate_file_queue_with_src_map(
            &import_src,
            cache,
            src_map,
        )?);
    }

    // only add itself after imports are added to satisfy borrow checker
    file_queue.push(file);

    // depth first
    Ok(file_queue.into_iter().rev().collect())
}

// takes a src, reads it, and parses the contents
fn parse_file(
    src: &Spanned<Intern<Src>>,
    cache: &mut SrcCache,
) -> Result<Spanned<File>, Vec<SpannedError>> {
    // technically this cache will save the full file in memory for the length of the program. I
    // have tested not using the cache here and it only saved about 5mb in a 100k LoC program, so I
    // felt it wasn't worth the downside of a file being potentially changed during compilation and
    // breaking error messages
    let assembly_code = cache
        .fetch(&src.val)
        .map_err(|error| {
            vec![
                SpannedError::new(src.span, "Error reading file")
                    .with_label(format!("{:?}", error)),
            ]
        })?
        .text();

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
