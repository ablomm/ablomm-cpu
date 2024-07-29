use std::collections::HashMap;
use chumsky::prelude::*;

use parser::*;

pub mod parser;
pub mod error;

pub fn assemble(assembly: &str) -> String {
    println!("{assembly}");
    match parser().parse(assembly){
        Ok(ast) => println!("{:?}", ast),
        Err(e) => {
println!("{:?}", e);


            print_syntax_error(&e);
        }
    }



    /*
    let mnemonic_asm_map = HashMap::from([("LD", ld_asm)]);
    let mut machine_code = String::new();

    for (line_index, line) in assembly.lines().enumerate() {
        let (mnemonic, params) = line.split_once(char::is_whitespace).unwrap();
        let machine_code_line = mnemonic_asm_map
            .get(&*mnemonic.to_uppercase())
            .expect(&format!(
                "Unknown mnemonic \"{}\" on line {}",
                mnemonic,
                line_index + 1
            ))(&params);
        machine_code.push_str(&format!("{machine_code_line:x}"));
    }

    println!("{}", machine_code);
    return machine_code;
*/
    return "abc".to_string();
}

// pub fn gen_label_addresses(assmebly: &str) -> iter::Map<String, u16> {}

pub fn ld_asm(params: &str) -> u32 {
    println!("ld with params: {}", params);

    return 123;
}
