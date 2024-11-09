use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

pub fn generate_int(
    operation: &Operation,
    _symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    let mut opcode: u32 = 0;

    if operation.parameters.len() != 0 {
        return Err("Expected INT with 0 parameters");
    }

    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);
    opcode |= Mnemonic::INT.generate();
    return Ok(opcode);
}
