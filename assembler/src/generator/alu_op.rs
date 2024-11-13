use crate::error::*;
use crate::generator::Generatable;
use crate::parser::*;
use std::collections::HashMap;

use super::seperate_modifiers;

pub mod unary_alu_op;

pub fn generate_alu_op(
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    let (conditions, alu_modifiers) = seperate_modifiers(&operation.full_mnemonic.modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            "Multiple conditions is not supported",
            conditions[1].span,
        ));
    }
    if alu_modifiers.len() > 1 {
        return Err(Error::new(
            "Multiple alu modifiers is not supported",
            alu_modifiers[1].span,
        ));
    }

    let mut opcode: u32 = 0;
    opcode |= conditions.generate();
    opcode |= alu_modifiers.generate();

    opcode |= operation.full_mnemonic.mnemonic.generate();
    if operation.parameters.len() == 2 {
        return generate_alu_op_2(operation, opcode, symbol_table);
    } else if operation.parameters.len() == 3 {
        return generate_alu_op_3(operation, opcode, symbol_table);
    } else {
        return Err(Error::new(
            "Expected 2 or 3 parameters",
            operation.parameters.span,
        ));
    }
}

// parameter length 2
fn generate_alu_op_2(
    operation: &Spanned<Operation>,
    opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_2_reg(register, opcode, operation, symbol_table)
        }
        Parameter::Number(number) => return generate_alu_op_2_num(*number, opcode, operation),
        _ => {
            return Err(Error::new(
                "Expected either a register or number",
                operation.parameters[0].span,
            ))
        }
    }
}

// when first parameter is a register
fn generate_alu_op_2_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= register.generate() << 12;
    opcode |= register.generate() << 8;

    match &operation.parameters[1].val {
        Parameter::Register(register2) => return generate_alu_op_2_reg_reg(register2, opcode),
        Parameter::Number(number) => return generate_alu_op_2_reg_num(*number, opcode),
        Parameter::Label(label) => {
            return generate_alu_op_2_reg_label(
                Spanned::new(label, operation.parameters[1].span),
                opcode,
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, number, or label",
                operation.parameters[1].span,
            ))
        }
    }
}

fn generate_alu_op_2_reg_reg(register2: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register2.generate() << 4;
    return Ok(opcode);
}

fn generate_alu_op_2_reg_num(number: u32, mut opcode: u32) -> Result<u32, Error> {
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_2_reg_label(
    label: Spanned<&str>,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= AluOpFlags::Immediate.generate();
    if let Some(label_line) = symbol_table.get(label.val) {
        opcode |= label_line & 0xff;
        return Ok(opcode);
    } else {
        return Err(Error::new("Could not find label", label.span));
    }
}

fn generate_alu_op_2_num(
    number: u32,
    mut opcode: u32,
    operation: &Spanned<Operation>,
) -> Result<u32, Error> {
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;

    match &operation.parameters[1].val {
        Parameter::Register(register) => return generate_alu_op_2_num_reg(register, opcode),
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[1].span,
            ))
        }
    }
}

fn generate_alu_op_2_num_reg(register: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register.generate() << 12;
    opcode |= register.generate() << 8;
    return Ok(opcode);
}

// parameter length 3
fn generate_alu_op_3(
    operation: &Spanned<Operation>,
    opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_3_reg(register, opcode, operation, symbol_table)
        }
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_alu_op_3_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= register.generate() << 12;

    match &operation.parameters[1].val {
        Parameter::Register(register2) => {
            return generate_alu_op_3_reg_reg(register2, opcode, operation, symbol_table)
        }
        Parameter::Number(number) => return generate_alu_op_3_reg_num(*number, opcode, operation),
        Parameter::Label(label) => {
            return generate_alu_op_3_reg_label(
                Spanned::new(label, operation.parameters[1].span),
                opcode,
                operation,
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, number, or label",
                operation.parameters[1].span,
            ))
        }
    }
}

fn generate_alu_op_3_reg_reg(
    register2: &Register,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= register2.generate() << 8;

    match &operation.parameters[2].val {
        Parameter::Register(register3) => return generate_alu_op_3_reg_reg_reg(register3, opcode),
        Parameter::Number(number) => return generate_alu_op_3_reg_reg_num(*number, opcode),
        Parameter::Label(label) => {
            return generate_alu_op_3_reg_reg_label(
                Spanned::new(label, operation.parameters[2].span),
                opcode,
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, number, or label",
                operation.parameters[2].span,
            ))
        }
    }
}

fn generate_alu_op_3_reg_reg_reg(register3: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register3.generate() << 4;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_reg_num(number: u32, mut opcode: u32) -> Result<u32, Error> {
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_reg_label(
    label: Spanned<&str>,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= AluOpFlags::Immediate.generate();
    if let Some(label_line) = symbol_table.get(label.val) {
        opcode |= label_line & 0xff;
        return Ok(opcode);
    } else {
        return Err(Error::new("Could not find label", label.span));
    }
}

fn generate_alu_op_3_reg_num(
    number: u32,
    mut opcode: u32,
    operation: &Spanned<Operation>,
) -> Result<u32, Error> {
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;

    match &operation.parameters[2].val {
        Parameter::Register(register2) => return generate_alu_op_3_reg_num_reg(register2, opcode),
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[2].span,
            ))
        }
    }
}

fn generate_alu_op_3_reg_num_reg(register2: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_label(
    label: Spanned<&str>,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    if let Some(label_line) = symbol_table.get(label.val) {
        opcode |= label_line & 0xff;
    } else {
        return Err(Error::new("Could not find label", label.span));
    }

    match &operation.parameters[2].val {
        Parameter::Register(register2) => generate_alu_op_3_reg_label_reg(register2, opcode),
        _ => return Err(Error::new("Expected a register", label.span)),
    }
}

fn generate_alu_op_3_reg_label_reg(register2: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}
