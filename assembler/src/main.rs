use ablomm_asm::*;
use ariadne::sources;
use clap::Parser;
use internment::Intern;
use std::fs;

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
    let assembly_input = fs::read_to_string(&args.input).expect("Error reading file");
    let input = Intern::new(args.input);

    let mut cache = sources(std::iter::once((input, &assembly_input)));

    match assemble(&assembly_input, input) {
        Ok(machine_code) => match args.output {
            Some(output_file) => {
                fs::write(output_file, machine_code).expect("Error writing file");
            }
            None => {
                print!("{}", machine_code);
            }
        },
        Err(errors) => errors.iter().for_each(|error| {
            error.eprint(&mut cache).ok();
        }),
    }
}
