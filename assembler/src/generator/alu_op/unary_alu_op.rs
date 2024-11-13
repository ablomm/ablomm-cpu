use crate::generator::*;
use std::collections::HashMap;

pub fn generate_unary_alu_op(
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
    if operation.parameters.len() == 1 {
        return generate_unary_alu_op_1(operation, opcode);
    } else if operation.parameters.len() == 2 {
        return generate_unary_alu_op_2(operation, opcode, symbol_table);
    } else {
        return Err(Error::new(
            "Expected 1 or 2 parameters",
            operation.parameters.span,
        ));
    }
}

fn generate_unary_alu_op_1(operation: &Spanned<Operation>, opcode: u32) -> Result<u32, Error> {
    match &operation.parameters[0].val {
        Parameter::Register(register) => return generate_unary_alu_op_1_reg(register, opcode),
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_unary_alu_op_1_reg(register: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register.generate() << 12;
    opcode |= register.generate();

    return Ok(opcode);
}

fn generate_unary_alu_op_2(
    operation: &Spanned<Operation>,
    opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_unary_alu_op_2_reg(register, opcode, operation, symbol_table)
        }
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_unary_alu_op_2_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= register.generate() << 12;

    match &operation.parameters[1].val {
        Parameter::Register(register2) => {
            return generate_unary_alu_op_2_reg_reg(register2, opcode)
        }
        Parameter::Label(label) => {
            return generate_unary_alu_op_2_reg_label(
                Spanned::new(label, operation.parameters[1].span),
                opcode,
                symbol_table,
            )
        }
        Parameter::Number(number) => return generate_unary_alu_op_2_reg_number(*number, opcode),
        _ => {
            return Err(Error::new(
                "Expected either a register, label, or number",
                operation.parameters[1].span,
            ))
        }
    }
}

fn generate_unary_alu_op_2_reg_reg(register2: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= register2.generate();
    return Ok(opcode);
}

fn generate_unary_alu_op_2_reg_label(
    label: Spanned<&str>,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    if let Some(label_line) = symbol_table.get(label.val) {
        opcode |= label_line & 0xff;
    } else {
        return Err(Error::new("Could not find label", label.span));
    }

    return Ok(opcode);
}

fn generate_unary_alu_op_2_reg_number(number: u32, mut opcode: u32) -> Result<u32, Error> {
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= number & 0xff;
    return Ok(opcode);
}
