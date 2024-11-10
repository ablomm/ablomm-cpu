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

    match &operation.parameters[0] {
        Parameter::Register(register, _) => {
            return generate_alu_op_2_reg(register, opcode, operation, symbol_table)
        }
        Parameter::Number(number, _) => return generate_alu_op_2_num(*number, opcode, operation),
        _ => return Err("Expected ALU Op to have a first parameter of either register or number"),
    }
}

// when first parameter is a register
fn generate_alu_op_2_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= register.generate() << 12;
    opcode |= register.generate() << 8;

    match &operation.parameters[1] {
        Parameter::Register(register2, _) => return generate_alu_op_2_reg_reg(register2, opcode),
        Parameter::Number(number, _) => return generate_alu_op_2_reg_num(*number, opcode),
        Parameter::Label(label, _) => {
            return generate_alu_op_2_reg_label(label, opcode, symbol_table)
        }
        _ => return Err("Could not find label in LD"),
    }
}

fn generate_alu_op_2_reg_reg(register2: &Register, mut opcode: u32) -> Result<u32, &'static str> {
    opcode |= register2.generate() << 4;
    return Ok(opcode);
}

fn generate_alu_op_2_reg_num(number: u32, mut opcode: u32) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_2_reg_label(
    label: &str,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Immediate.generate();
    if let Some(label_line) = symbol_table.get(label) {
        opcode |= label_line & 0xff;
        return Ok(opcode);
    } else {
        return Err("Could not find label in LD");
    }
}

fn generate_alu_op_2_num(
    number: u32,
    mut opcode: u32,
    operation: &Operation,
) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;

    match &operation.parameters[1] {
        Parameter::Register(register, _) => return generate_alu_op_2_num_reg(register, opcode),
        _ => return Err("Expected ALU Op to have second parameter of register"),
    }
}

fn generate_alu_op_2_num_reg(register: &Register, mut opcode: u32) -> Result<u32, &'static str> {
    opcode |= register.generate() << 12;
    opcode |= register.generate() << 8;
    return Ok(opcode);
}

// parameter length 3
pub fn generate_alu_op_3(
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    let mut opcode: u32 = 0;
    opcode |= operation.full_mnemonic.modifiers.generate();
    opcode |= operation.full_mnemonic.mnemonic.generate();

    match &operation.parameters[0] {
        Parameter::Register(register, _) => {
            return generate_alu_op_3_reg(register, opcode, operation, symbol_table)
        }
        _ => return Err("Expected ALU Op to have first parameter of register"),
    }
}

fn generate_alu_op_3_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= register.generate() << 12;

    match &operation.parameters[1] {
        Parameter::Register(register2, _) => {
            return generate_alu_op_3_reg_reg(register2, opcode, operation, symbol_table)
        }
        Parameter::Number(number, _) => {
            return generate_alu_op_3_reg_num(*number, opcode, operation)
        }
        Parameter::Label(label, _) => {
            return generate_alu_op_3_reg_label(label, opcode, operation, symbol_table)
        }
        _ => {
            return Err(
                "Expected ALU op to have third parameters of either register or immediate or label",
            )
        }
    }
}

fn generate_alu_op_3_reg_reg(
    register2: &Register,
    mut opcode: u32,
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= register2.generate() << 8;

    match &operation.parameters[2] {
        Parameter::Register(register3, _) => {
            return generate_alu_op_3_reg_reg_reg(register3, opcode)
        }
        Parameter::Number(number, _) => return generate_alu_op_3_reg_reg_num(*number, opcode),
        Parameter::Label(label, _) => {
            return generate_alu_op_3_reg_reg_label(label, opcode, symbol_table)
        }
        _ => {
            return Err(
                "Expected ALU op to have third parameters of either register, number, or label",
            )
        }
    }
}

fn generate_alu_op_3_reg_reg_reg(
    register3: &Register,
    mut opcode: u32,
) -> Result<u32, &'static str> {
    opcode |= register3.generate() << 4;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_reg_num(number: u32, mut opcode: u32) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_reg_label(
    label: &str,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Immediate.generate();
    if let Some(label_line) = symbol_table.get(label) {
        opcode |= label_line & 0xff;
        return Ok(opcode);
    } else {
        return Err("Could not find label in LD");
    }
}

fn generate_alu_op_3_reg_num(
    number: u32,
    mut opcode: u32,
    operation: &Operation,
) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;

    match &operation.parameters[2] {
        Parameter::Register(register2, _) => {
            return generate_alu_op_3_reg_num_reg(register2, opcode)
        }
        _ => return Err("Expected ALU to have third parameter of register"),
    }
}

fn generate_alu_op_3_reg_num_reg(
    register2: &Register,
    mut opcode: u32,
) -> Result<u32, &'static str> {
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_label(
    label: &str,
    mut opcode: u32,
    operation: &Operation,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, &'static str> {
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    if let Some(label_line) = symbol_table.get(label) {
        opcode |= label_line & 0xff;
    } else {
        return Err("Could not find label in LD");
    }

    match &operation.parameters[2] {
        Parameter::Register(register2, _) => generate_alu_op_3_reg_label_reg(register2, opcode),
        _ => return Err("Expected ALU to have third parameter of register"),
    }
}

fn generate_alu_op_3_reg_label_reg(
    register2: &Register,
    mut opcode: u32,
) -> Result<u32, &'static str> {
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}
