use ablomm_asm::error::Error;
use clap::Parser;
use std::{fs, process::ExitCode};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file input
    input: String,

    /// file output
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match ablomm_asm::assemble(&args.input) {
        Ok(machine_code) => {
            match args.output {
                Some(output_file) => {
                    fs::write(output_file, machine_code).expect("Error writing file");
                }
                None => {
                    print!("{}", machine_code);
                }
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            match error {
                Error::Spanned(errors, mut cache) => errors.iter().for_each(|error| {
                    error.eprint(&mut cache).ok();
                }),
                Error::Bare(error) => eprintln!("{}", error),
            }

            ExitCode::FAILURE
        }
    }
}
