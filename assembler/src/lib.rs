use std::{fs, io, path::Path};

use ariadne::{Cache, FnCache};

use error::{ATTENTION_COLOR, Error, SpannedError};
use internment::Intern;
use span::{Span, Spanned};
use src::Src;

use crate::error::{RecoveredError, RecoveredResult};

mod ast;
pub mod error;
mod expression;
mod file;
mod generator;
mod parser;
mod span;
mod src;
mod symbol_table;

pub type SrcCache = FnCache<Intern<Src>, fn(&Intern<Src>) -> io::Result<String>, String>;

// error includes cache in order to print errors without re-reading files
// error includes recovered machine_code
#[allow(clippy::type_complexity)]
pub fn assemble(
    src: &str,
) -> RecoveredResult<Vec<u32>, Vec<u32>, (Vec<Error>, impl Cache<Intern<Src>>)> {
    // cache of file name and corresponding file contents, used to
    // associate file names to contents for printing errors
    let mut cache: SrcCache = FnCache::new(|src: &Intern<Src>| fs::read_to_string(src.as_path()));

    // fails if file not found
    // this is the root file, given in the command-line argument
    let src = Intern::new(
        match Src::new(Path::new(src).to_path_buf())
            .map_err(|error| Error::Bare(format!("Error in provided file \"{}\": {}", src, error)))
        {
            Ok(src) => src,
            Err(error) => {
                // have to do ths in a match instead of map_err because cache is moved
                return Err(RecoveredError(Vec::new(), (vec![error], cache)));
            }
        },
    );

    // create a dummy span because there is no actual span for the root file, as it doesn't have a
    // corresponding import statement
    let dummy_span = Span::new(src, 0..0);
    let src = Spanned::new(src, dummy_span);

    let mut errors = Vec::new();

    let mut ast = match src.build_ast(&mut cache) {
        Ok(ast) => ast,
        Err(RecoveredError(ast, mut file_errors)) => {
            errors.append(&mut file_errors);
            ast
        }
    };

    match ast.init_symbol_tables() {
        Ok(_) => (),
        Err(mut symbol_table_errors) => errors.append(&mut symbol_table_errors),
    }

    let machine_code = match ast.generate() {
        Ok(machine_code) => machine_code,
        Err(RecoveredError(machine_code, mut generation_errors)) => {
            errors.append(&mut generation_errors);
            machine_code
        }
    };

    if errors.is_empty() {
        Ok(machine_code)
    } else {
        Err(RecoveredError(
            machine_code,
            (errors.into_iter().map(Error::Spanned).collect(), cache),
        ))
    }
}
