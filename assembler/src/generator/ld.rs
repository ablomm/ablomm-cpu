use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

pub fn generate_ld(
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    if operation.parameters.len() != 2 {
        return Err("Expected LD with 2 parameters");
    }
    let mut opcode: u32 = 0;
    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);

    if let Parameter::Register(register) = operation.parameters[0] {
        if let Parameter::Indirect(parameter) = &operation.parameters[1] {
            if let Parameter::Register(register2) = **parameter {
                //LDR
                opcode |= Mnemonic::LDR.generate();
                opcode |= register.generate() << 16;
                opcode |= register2.generate() << 12;
                return Ok(opcode);
            } else if let Parameter::Number(number) = **parameter {
                //LD
                opcode |= Mnemonic::LD.generate();
                opcode |= register.generate() << 16;
                opcode |= number & 0xffff;
                return Ok(opcode);
            } else if let Parameter::Label(label) = &**parameter {
                //LD
                opcode |= Mnemonic::LD.generate();
                opcode |= register.generate() << 16;
                if let Some(label_line) = symbol_table.get(&*label) {
                    opcode |= label_line & 0xffff;
                    return Ok(opcode);
                } else {
                    return Err("Could not find label in LD");
                }
            } else {
                return Err("LD only supports indirect constants, registers, and labels");
            }
        } else if let Parameter::Register(register2) = operation.parameters[1] {
            //MOV
            opcode |= Mnemonic::PASSA.generate();
            opcode |= register.generate() << 12;
            opcode |= register2.generate() << 8;
            return Ok(opcode);
        } else if let Parameter::Number(number) = operation.parameters[1] {
            //LDI
            opcode |= Mnemonic::LDI.generate();
            opcode |= register.generate() << 16;
            opcode |= number & 0xffff;
            return Ok(opcode);
        } else if let Parameter::Label(label) = &operation.parameters[1] {
            //LDI
            opcode |= Mnemonic::LDI.generate();
            opcode |= register.generate() << 16;
            if let Some(label_line) = symbol_table.get(&*label) {
                opcode |= label_line & 0xffff;
                return Ok(opcode);
            } else {
                return Err("Could not find label in LD");
            }
        } else {
            return Err(
                "Expected second LD parameter to be either an indirect, register, or number",
            );
        }
    } else {
        return Err("Expected first LD parameter to be a register");
    }
}
