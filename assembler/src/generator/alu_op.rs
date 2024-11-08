use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

pub fn generate_alu_op(
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    if operation.parameters.len() == 2 {
        return generate_alu_op_2(operation, symbol_table);
    } else if operation.parameters.len() == 3 {
        return generate_alu_op_3(operation, symbol_table);
    } else {
        return Err("Expected ALU op with 2 or 3 parameters");
    }
}

// parameter length 2
pub fn generate_alu_op_2(
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    let mut opcode: u32 = 0;
    opcode |= operation.full_mnemonic.modifiers.generate();
    opcode |= operation.full_mnemonic.mnemonic.generate();
    if let Parameter::Register(register) = operation.parameters[0] {
        if let Parameter::Register(register2) = operation.parameters[1] {
            opcode |= register.generate() << 12;
            opcode |= register.generate() << 8;
            opcode |= register2.generate() << 4;
            return Ok(opcode);
        } else if let Parameter::Number(number) = operation.parameters[1] {
            opcode |= register.generate() << 12;
            opcode |= register.generate() << 8;
            opcode |= AluOpFlags::Immediate.generate();
            opcode |= number & 0xff;
            return Ok(opcode);
        } else if let Parameter::Label(label) = &operation.parameters[1] {
            opcode |= register.generate() << 12;
            opcode |= register.generate() << 8;
            opcode |= AluOpFlags::Immediate.generate();
            if let Some(label_line) = symbol_table.get(&*label) {
                opcode |= label_line & 0xff;
                return Ok(opcode);
            } else {
                return Err("Could not find label in LD");
            }
        } else {
            return Err(
                "Expected ALU op to have second parameters of either register, number, or label",
            );
        }
    } else {
        return Err("Expected ALU Op to have first parameter of register");
    }
}

pub fn generate_alu_op_3(
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    let mut opcode: u32 = 0;
    opcode |= operation.full_mnemonic.modifiers.generate();
    opcode |= operation.full_mnemonic.mnemonic.generate();
    if let Parameter::Register(register) = operation.parameters[0] {
        if let Parameter::Register(register2) = operation.parameters[1] {
            if let Parameter::Register(register3) = operation.parameters[2] {
                opcode |= register.generate() << 12;
                opcode |= register2.generate() << 8;
                opcode |= register3.generate() << 4;
                return Ok(opcode);
            } else if let Parameter::Number(number) = operation.parameters[2] {
                opcode |= register.generate() << 12;
                opcode |= register2.generate() << 8;
                opcode |= AluOpFlags::Immediate.generate();
                opcode |= number & 0xff;
                return Ok(opcode);
            } else if let Parameter::Label(label) = &operation.parameters[2] {
                opcode |= register.generate() << 12;
                opcode |= register2.generate() << 8;
                opcode |= AluOpFlags::Immediate.generate();
                if let Some(label_line) = symbol_table.get(&*label) {
                    opcode |= label_line & 0xff;
                    return Ok(opcode);
                } else {
                    return Err("Could not find label in LD");
                }
            } else {
                return Err(
                    "Expected ALU op to have third parameters of either register, number, or label",
                );
            }
        } else if let Parameter::Number(number) = operation.parameters[1] {
            opcode |= AluOpFlags::Reverse.generate();
            opcode |= AluOpFlags::Immediate.generate();

            if let Parameter::Register(register2) = operation.parameters[2] {
                opcode |= register.generate() << 12;
                opcode |= register2.generate() << 8;
                opcode |= number & 0xff;
                return Ok(opcode);
            } else {
                return Err("Expected ALU to have third parameter of register");
            }
        } else if let Parameter::Label(label) = &operation.parameters[1] {
            opcode |= AluOpFlags::Reverse.generate();
            opcode |= AluOpFlags::Immediate.generate();

            if let Parameter::Register(register2) = operation.parameters[2] {
                if let Some(label_line) = symbol_table.get(&*label) {
                    opcode |= register.generate() << 12;
                    opcode |= register2.generate() << 8;
                    opcode |= label_line & 0xff;
                    return Ok(opcode);
                } else {
                    return Err("Could not find label in LD");
                }
            } else {
                return Err("Expected ALU to have third parameter of register");
            }
        } else {
            return Err(
                "Expected ALU op to have third parameters of either register or immediate or label",
            );
        }
    } else {
        return Err("Expected ALU Op to have first parameter of register");
    }
}
