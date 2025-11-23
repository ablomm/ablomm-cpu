use std::{fs, io, path::Path};

use ariadne::{Cache, FnCache};
use ast::Ast;

use error::{ATTENTION_COLOR, Error, SpannedError};
use internment::Intern;
use span::{Span, Spanned};
use src::Src;
use symbol_table::setup as st_setup;

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

// return a string which is the machine code
// error includes cache in order to print errors without re-reading files
// error includes recovered machine_code
pub fn assemble(src: &str) -> RecoveredResult<Vec<u32>, Vec<u32>, Error<impl Cache<Intern<Src>>>> {
    // fails if file not found
    // this is the root file, given in the command-line argument
    let src = Intern::new(Src::new(Path::new(src).to_path_buf()).map_err(|error| {
        RecoveredError(
            Vec::new(),
            Error::Bare(format!("Error in provided file \"{}\": {}", src, error)),
        )
    })?);

    // create a dummy span because there is no actual span for the root file, as it doesn't have a
    // corresponding import statement
    let dummy_span = Span::new(src, 0..0);
    let src = Spanned::new(src, dummy_span);

    let mut errors = Vec::new();

    // cache of file name and corresponding file contents, used to
    // associate file names to contents for printing errors
    let mut cache: SrcCache = FnCache::new(|src: &Intern<Src>| fs::read_to_string(src.as_path()));

    // file queue is order in which to generate machine code
    // can't do map_err because of borrow checker :(
    let mut file_queue = match file::generate_file_queue(src, &mut cache) {
        Ok(file_queue) => file_queue,
        Err(RecoveredError(file_queue, mut file_errors)) => {
            errors.append(&mut file_errors);
            file_queue
        }
    };

    match st_setup::init_symbol_tables(&mut file_queue) {
        Ok(_) => (),
        Err(mut symbol_table_errors) => errors.append(&mut symbol_table_errors),
    }

    let ast = Ast { files: file_queue };

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
        Err(RecoveredError(machine_code, Error::Spanned(errors, cache)))
    }
}
