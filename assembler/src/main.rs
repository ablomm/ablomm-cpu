use clap::Parser;
use std::fs;
use ablomm_asm::*;

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
    let assembly_input = fs::read_to_string(args.input).expect("Error reading file");

    match assemble(&assembly_input) {
        Ok(machine_code) => match args.output {
            Some(output_file) => {
                fs::write(output_file, machine_code).expect("Error writing file");
            }
            None => {
                print!("{}", machine_code);
            }
        },
        Err(error) => panic!("{}", error),
    }
}
