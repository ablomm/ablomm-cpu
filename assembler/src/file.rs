use ariadne::Cache;
use chumsky::prelude::*;
use std::collections::HashSet;

use internment::Intern;

use crate::{
    SrcCache,
    ast::{Ast, Block, File, Statement},
    error::{RecoveredError, RecoveredResult, SpannedError},
    parser,
    span::Spanned,
    src::{self, Src},
};

impl Spanned<Intern<Src>> {
    // takes Src and returns the full Ast assuming Src is the root file
    pub fn build_ast(&self, cache: &mut SrcCache) -> RecoveredResult<Ast> {
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

        let mut sub_files = match file.block.get_import_files(self.val, cache, src_map) {
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
                    vec![
                        SpannedError::new(self.span, "Error reading file")
                            .with_label(format!("{:?}", error)),
                    ],
                ));
            }
        };

        let (block, errors) = parser::file_block_parser()
            .parse(assembly_code.with_context(self.val))
            .into_output_errors();

        let errors: Vec<_> = errors.iter().map(|error| error.into()).collect();

        let file = block.map(|block| {
            block.span.spanned(File {
                src: self.val,
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
}
impl Block {
    // gets list of files for each import in the block
    pub fn get_import_files(
        &mut self,
        src: Intern<Src>,
        cache: &mut SrcCache,
        src_map: &mut HashSet<Intern<Src>>,
    ) -> RecoveredResult<Vec<Spanned<File>>> {
        let mut files = Vec::new();
        let mut errors = Vec::new();

        self.statements.retain_mut(|statement| {
            match statement.get_import_files(src, cache, src_map) {
                Ok(mut statement_files) => {
                    files.append(&mut statement_files);
                    true
                }
                Err(RecoveredError(mut statement_files, mut statement_errors)) => {
                    // need the condition before append because append will clear out the lists
                    if statement_files.is_empty() {
                        files.append(&mut statement_files);
                        errors.append(&mut statement_errors);

                        // this import, for whatever reason, did not generate a file (e.g. os read error),
                        // so we need to remove the import so we don't try to use it in subsequent steps
                        // which could cause phantom errors
                        false
                    } else {
                        files.append(&mut statement_files);
                        errors.append(&mut statement_errors);
                        true
                    }
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

impl Spanned<Statement> {
    // get list of files that the statement is importing
    pub fn get_import_files(
        &mut self,
        src: Intern<Src>,
        cache: &mut SrcCache,
        src_map: &mut HashSet<Intern<Src>>,
    ) -> RecoveredResult<Vec<Spanned<File>>> {
        match &mut self.val {
            Statement::Import(import) => match src::get_import_src(src, import) {
                Ok(import_src) => {
                    let import_src = import.file.span_to(import_src);
                    if src_map.contains(&import_src) {
                        // we already did this import
                        Ok(Vec::new())
                    } else {
                        import_src.build_file_queue(cache, src_map)
                    }
                }
                Err(error) => Err(RecoveredError(
                    Vec::new(),
                    vec![
                        SpannedError::new(import.file.span, "Error finding file")
                            .with_label(error.to_string()),
                    ],
                )),
            },
            Statement::Block(block) => block.get_import_files(src, cache, src_map),
            _ => Ok(Vec::new()),
        }
    }
}
