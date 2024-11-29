use ablomm_asm::*;
use ariadne::sources;
use clap::Parser;
use internment::Intern;
use std::{fs, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file input
    input: String,

    /// file output
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    match assemble(&args.input) {
        Ok(machine_code) => match args.output {
            Some(output_file) => {
                fs::write(output_file, machine_code).expect("Error writing file");
            }
            None => {
                print!("{}", machine_code);
            }
        },
        Err((errors, mut cache)) => {
            errors.iter().for_each(|error| {
                error.eprint(&mut cache).ok();
            });
            process::exit(-1);
        }
    }
}
