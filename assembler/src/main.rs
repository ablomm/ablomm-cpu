use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let assembly_file =
        fs::read_to_string(args.get(1).expect("Missing source file argument")).expect("Error reading file");


    let machine_code = ablomm_asm::assemble(&assembly_file);
}
