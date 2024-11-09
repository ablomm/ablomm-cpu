use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

pub fn generate_pop(
    operation: &Operation,
    _symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    let mut opcode: u32 = 0;
    if operation.parameters.len() != 1 {
        return Err("Expected POP with 1 parameter");
    }
    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);

    if let Parameter::Register(register) = operation.parameters[0] {
        opcode |= Mnemonic::POP.generate();
        opcode |= register.generate() << 20;
        return Ok(opcode);
    } else {
        return Err("Expected POP parameter to be a register");
    }
}
