use ariadne::Cache;
use chumsky::prelude::*;
use std::collections::HashSet;

use internment::Intern;

use crate::{
    SrcCache,
    ast::{Ast, Block, File, Statement},
    error::{Error, RecoveredError, RecoveredResult, SpannedError},
    parser,
    span::Spanned,
    src::Src,
};

impl Spanned<Intern<Src>> {
    // takes Src and returns the full Ast assuming Src is the root file
    pub(super) fn build_ast(&self, cache: &mut SrcCache) -> RecoveredResult<Ast> {
        let files = self
            .build_file_queue(cache, &mut HashSet::new())
            .map_err(|RecoveredError(files, errors)| RecoveredError(Ast { files }, errors))?;

        Ok(Ast { files })
    }

    // gets all the files for a given Src and returns it in the correct order for generation
    fn build_file_queue(
        &self,
        cache: &mut SrcCache,
        src_map: &mut HashSet<Intern<Src>>,
    ) -> RecoveredResult<Vec<Spanned<File>>> {
        let mut file_queue = Vec::new();
        let mut errors = Vec::new();

        let mut file = match self.parse(cache) {
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

        src_map.insert(self.val);

        let mut sub_files = match file.as_mut_ref().get_import_files(cache, src_map) {
            Ok(sub_files) => sub_files,
            Err(RecoveredError(sub_files, mut file_errors)) => {
                errors.append(&mut file_errors);
                sub_files
            }
        };

        // depth first
        file_queue.push(file);
        file_queue.append(&mut sub_files);

        if errors.is_empty() {
            Ok(file_queue)
        } else {
            Err(RecoveredError(file_queue, errors))
        }
    }

    fn parse(&self, cache: &mut SrcCache) -> RecoveredResult<Spanned<File>, Option<Spanned<File>>> {
        // technically this cache will save the full file in memory for the length of the program. I
        // have tested not using the cache here and it only saved about 5mb in a 100k LoC program, so I
        // felt it wasn't worth the downside of a file being potentially changed during compilation and
        // breaking error messages
        let assembly_code = match cache.fetch(&self.val) {
            Ok(assembly_code) => assembly_code.text(),
            Err(error) => {
                return Err(RecoveredError(
                    None,
                    vec![Error::Spanned(Box::new(
                        SpannedError::new(self.span, "Error reading file")
                            .with_label(format!("{:?}", error)),
                    ))],
                ));
            }
        };

        let (file, errors) = parser::file_parser()
            .spanned()
            .parse(assembly_code.with_context(self.val))
            .into_output_errors();

        let errors: Vec<_> = errors
            .iter()
            .map(|error| Error::Spanned(Box::new(error.into())))
            .collect();

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
}

impl Spanned<&mut File> {
    fn get_import_files(
        &mut self,
        cache: &mut SrcCache,
        src_map: &mut HashSet<Intern<Src>>,
    ) -> RecoveredResult<Vec<Spanned<File>>> {
        self.span
            .spanned(&mut self.block)
            .get_import_files(cache, src_map)
    }
}

impl Spanned<&mut Block> {
    // gets list of files for each import in the block
    fn get_import_files(
        &mut self,
        cache: &mut SrcCache,
        src_map: &mut HashSet<Intern<Src>>,
    ) -> RecoveredResult<Vec<Spanned<File>>> {
        let mut files = Vec::new();
        let mut errors = Vec::new();

        self.statements.retain_mut(|statement| {
            match statement.as_mut_ref().get_import_files(cache, src_map) {
                Ok(mut statement_files) => {
                    files.append(&mut statement_files);
                    true
                }
                Err(RecoveredError(mut statement_files, mut statement_errors)) => {
                    // false if this import, for whatever reason, did not generate a file (e.g. os read error),
                    // so we need to remove the import so we don't try to use it in subsequent steps
                    let should_retain_statement =
                        matches!(statement.val, Statement::Import(_)) && statement_files.is_empty();
                    files.append(&mut statement_files);
                    errors.append(&mut statement_errors);
                    should_retain_statement
                }
            }
        });

        if errors.is_empty() {
            Ok(files)
        } else {
            Err(RecoveredError(files, errors))
        }
    }
}

impl Spanned<&mut Statement> {
    // get list of files that the statement is importing
    fn get_import_files(
        &mut self,
        cache: &mut SrcCache,
        src_map: &mut HashSet<Intern<Src>>,
    ) -> RecoveredResult<Vec<Spanned<File>>> {
        match &mut self.val {
            Statement::Import(import) => {
                if src_map.contains(&import.src) {
                    // we already did this import
                    Ok(Vec::new())
                } else {
                    import.src.build_file_queue(cache, src_map)
                }
            }
            Statement::Block(block) => self.span.spanned(block).get_import_files(cache, src_map),
            _ => Ok(Vec::new()),
        }
    }
}
