use std::{collections::HashMap, path::Path};

use ariadne::{Cache, sources};
use ast::Ast;

use error::{ATTENTION_COLOR, Error, SpannedError};
use internment::Intern;
use span::{Span, Spanned};
use src::Src;
use symbol_table::setup as st_setup;

mod ast;
pub mod error;
mod expression;
mod file;
mod generator;
mod parser;
mod span;
mod src;
mod symbol_table;

// return a string which is the machine code
// error includes cache in order to print errors without re-reading files
pub fn assemble(src: &str) -> Result<String, Error<impl Cache<Intern<Src>>>> {
    // fails if file not found
    // this is the root file, given in the command-line argument
    let src =
        Intern::new(Src::new(Path::new(src).to_path_buf()).map_err(|error| {
            Error::Bare(format!("Error in provided file \"{}\": {}", src, error))
        })?);

    // create a dummy span because there is no actual span for the root file, as it doesn't have a
    // corresponding import statement
    let dummy_span = Span::new(src, 0..0);
    let src = Spanned::new(src, dummy_span);

    // cache of file name and corresponding file contents, used to
    // print error messages and check if a file has already been parsed
    let mut cache = HashMap::new();

    // file queue is order in which to generate symbol tables
    // can't do map_err because of borrow checker
    let mut file_queue = match file::generate_file_queue(&src, &mut cache) {
        Ok(file_queue) => file_queue,
        Err(error) => return Err(Error::Spanned(error, sources(cache))),
    };

    // needed to get types (and as much values as possible without addresses) because the number of
    // words for a statement may depend on the type and value of symbols
    match st_setup::init_symbol_tables(&mut file_queue) {
        Ok(_) => (),
        Err(error) => return Err(Error::Spanned(vec![error], sources(cache))),
    }

    let ast = Ast {
        // reverse order to get correct generation order, which is opposite of symbol table
        // creation order (i.e. had to create imported file's symbol table before itself)
        files: file_queue.into_iter().rev().collect(),
    };

    generator::compile_ast(&ast).map_err(|error| Error::Spanned(error, sources(cache)))
}
