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
            match &args.output {
                Some(output_file) => match fs::write(output_file, machine_code) {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("Error while writing to file \"{}\": {}", output_file, error);
                        return ExitCode::FAILURE;
                    }
                },
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
