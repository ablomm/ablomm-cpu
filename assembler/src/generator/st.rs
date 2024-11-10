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

    match &operation.parameters[0] {
        Parameter::Register(register, _) => {
            return generate_st_reg(register, opcode, operation, symbol_table)
        }

        _ => return Err("Expected first LD parameter to be a register"),
    }
}

fn generate_st_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= register.generate() << 16;

    match &operation.parameters[1] {
        Parameter::Register(register2, _) => {
            return generate_st_reg_reg(register, &register2, opcode)
        }
        Parameter::Indirect(parameter, _) => {
            return generate_st_reg_indirect(parameter, opcode, symbol_table)
        }

        _ => return Err("Expected second ST parameter to be either an indirect or register"),
    }
}

fn generate_st_reg_reg(
    register: &Register,
    register2: &Register,
    mut opcode: u32,
) -> Result<u32, &'static str> {
    // MOVR
    opcode |= Mnemonic::PASSA.generate();
    opcode &= !(0b1111 << 16); // need to zero out previously set register
    opcode |= register2.generate() << 12;
    opcode |= register.generate() << 8;
    return Ok(opcode);
}

fn generate_st_reg_indirect(
    parameter: &Parameter,
    opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    match parameter {
        Parameter::Register(register2, _) => return generate_st_reg_ireg(register2, opcode),
        Parameter::Number(number, _) => return generate_st_reg_inum(*number, opcode),
        Parameter::Label(label, _) => return generate_st_reg_ilabel(label, opcode, symbol_table),
        _ => return Err("ST only supports indirect constants, registers, and labels"),
    }
}

fn generate_st_reg_ireg(register2: &Register, mut opcode: u32) -> Result<u32, &'static str> {
    // STR
    opcode |= Mnemonic::STR.generate();
    opcode |= register2.generate() << 12;
    return Ok(opcode);
}

fn generate_st_reg_inum(number: u32, mut opcode: u32) -> Result<u32, &'static str> {
    // ST
    opcode |= Mnemonic::ST.generate();
    opcode |= number & 0xffff;
    return Ok(opcode);
}

fn generate_st_reg_ilabel(
    label: &str,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    // ST
    opcode |= Mnemonic::ST.generate();
    if let Some(label_line) = symbol_table.get(label) {
        opcode |= label_line & 0xffff;
        return Ok(opcode);
    } else {
        return Err("Could not find label in ST");
    }
}
