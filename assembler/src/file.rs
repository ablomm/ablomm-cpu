use ariadne::Cache;
use chumsky::prelude::*;
use std::collections::HashSet;

use internment::Intern;

use crate::{
    SrcCache,
    ast::{Block, File, Import, Statement},
    error::{RecoveredError, RecoveredResult, SpannedError},
    parser,
    span::Spanned,
    src::{self, Src},
};

// takes in a canonical file path, address, and cache and returns a queue of the order in which to
// generate the machine code
pub fn generate_file_queue(
    src: &Spanned<Intern<Src>>,
    cache: &mut SrcCache,
) -> RecoveredResult<Vec<Spanned<File>>> {
    generate_file_queue_with_src_map(src, cache, &mut HashSet::new())
}

fn generate_file_queue_with_src_map(
    src: &Spanned<Intern<Src>>,
    cache: &mut SrcCache,
    src_map: &mut HashSet<Intern<Src>>,
) -> RecoveredResult<Vec<Spanned<File>>> {
    let mut file_queue = Vec::new();
    let mut errors = Vec::new();

    let file = match parse_file(src, cache) {
        Ok(file) => file,
        Err(RecoveredError(file, mut parse_errors)) => {
            errors.append(&mut parse_errors);
            if let Some(file) = file {
                // recovered ast with errors
                file
            } else {
                return Err(RecoveredError(file_queue, errors));
            }
        }
    };

    src_map.insert(src.val);

    for import in file.block.get_imports() {
        let import_src = match src::get_import_src(src, import) {
            Ok(import_src) => import.file.span_to(import_src),
            Err(error) => {
                errors.push(
                    SpannedError::new(import.file.span, "Error finding file")
                        .with_label(error.to_string()),
                );

                // this import file doesn't exist, check next
                continue;
            }
        };

        if src_map.contains(&import_src.val) {
            // we already did this import
            continue;
        }

        let mut sub_files = match generate_file_queue_with_src_map(&import_src, cache, src_map) {
            Ok(sub_files) => sub_files,
            Err(RecoveredError(sub_files, mut sub_errors)) => {
                errors.append(&mut sub_errors);
                sub_files
            }
        };

        file_queue.append(&mut sub_files);
    }

    // only add itself after imports are added to satisfy borrow checker
    file_queue.push(file);

    // depth first, so reverse the order
    let file_queue = file_queue.into_iter().rev().collect();

    if errors.is_empty() {
        Ok(file_queue)
    } else {
        Err(RecoveredError(file_queue, errors))
    }
}

// takes a src, reads it, and parses the contents
fn parse_file(
    src: &Spanned<Intern<Src>>,
    cache: &mut SrcCache,
) -> RecoveredResult<Spanned<File>, Option<Spanned<File>>> {
    // technically this cache will save the full file in memory for the length of the program. I
    // have tested not using the cache here and it only saved about 5mb in a 100k LoC program, so I
    // felt it wasn't worth the downside of a file being potentially changed during compilation and
    // breaking error messages
    let assembly_code = match cache.fetch(&src.val) {
        Ok(assembly_code) => assembly_code.text(),
        Err(error) => {
            return Err(RecoveredError(
                None,
                vec![
                    SpannedError::new(src.span, "Error reading file")
                        .with_label(format!("{:?}", error)),
                ],
            ));
        }
    };

    let (block, errors) = parser::file_block_parser()
        .parse(assembly_code.with_context(src.val))
        .into_output_errors();

    let errors: Vec<_> = errors.iter().map(|error| error.into()).collect();

    let file = block.map(|block| {
        block.span.spanned(File {
            src: src.val,
            block: block.val,
        })
    });

    if let Some(file) = file {
        if errors.is_empty() {
            Ok(file)
        } else {
            Err(RecoveredError(Some(file), errors))
        }
    } else {
        Err(RecoveredError(file, errors))
    }
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
