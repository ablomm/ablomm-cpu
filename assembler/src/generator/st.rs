use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

pub fn generate_st(
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    if operation.parameters.len() != 2 {
        return Err("Expected ST with 2 parameters");
    }
    let mut opcode: u32 = 0;
    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);

    if let Parameter::Register(register, _) = operation.parameters[0] {
        if let Parameter::Indirect(parameter, _) = &operation.parameters[1] {
            if let Parameter::Register(register2, _) = **parameter {
                // STR
                opcode |= Mnemonic::STR.generate();
                opcode |= register.generate() << 16;
                opcode |= register2.generate() << 12;
                return Ok(opcode);
            } else if let Parameter::Number(number, _) = **parameter {
                // ST
                opcode |= Mnemonic::ST.generate();
                opcode |= register.generate() << 16;
                opcode |= number & 0xffff;
                return Ok(opcode);
            } else if let Parameter::Label(label, _) = &**parameter {
                // ST
                opcode |= Mnemonic::ST.generate();
                opcode |= register.generate() << 16;
                if let Some(label_line) = symbol_table.get(&*label) {
                    opcode |= label_line & 0xffff;
                    return Ok(opcode);
                } else {
                    return Err("Could not find label in ST");
                }
            } else {
                return Err("ST only supports indirect constants, registers, and labels");
            }
        } else if let Parameter::Register(register2, _) = operation.parameters[1] {
            // MOVR
            opcode |= Mnemonic::PASSA.generate();
            opcode |= register2.generate() << 12;
            opcode |= register.generate() << 8;
            return Ok(opcode);
        } else {
            return Err("Expected second ST parameter to be either an indirect or register");
        }
    } else {
        return Err("Expected first LD parameter to be a register");
    }
}
