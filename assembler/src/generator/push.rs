use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

pub fn generate_push(
    operation: &Operation,
    _symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    let mut opcode: u32 = 0;
    if operation.parameters.len() != 1 {
        return Err("Expected PUSH with 1 parameter");
    }
    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);

    if let Parameter::Register(register, _) = operation.parameters[0] {
        opcode |= Mnemonic::PUSH.generate();
        opcode |= register.generate() << 16;
        return Ok(opcode);
    } else {
        return Err("Expected PUSH parameter to be a register");
    }
}
