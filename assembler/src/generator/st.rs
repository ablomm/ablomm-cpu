use crate::generator::*;
use std::collections::HashMap;

pub fn generate_st(
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    if operation.parameters.len() != 2 {
        return Err(Error::new(
            "Expected 2 parameters",
            operation.parameters.span,
        ));
    }

    let (conditions, alu_modifiers) = seperate_modifiers(&operation.full_mnemonic.modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            "Multiple conditions is not supported",
            conditions[1].span,
        ));
    }
    if alu_modifiers.len() > 0 {
        return Err(Error::new(
            "Alu modifiers is not supported on this instruction",
            operation.full_mnemonic.modifiers.span,
        ));
    }

    let mut opcode: u32 = 0;
    opcode |= conditions.generate();

    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_st_reg(register, opcode, operation, symbol_table)
        }

        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_st_reg(
    register: &Register,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= register.generate() << 16;

    match &operation.parameters[1].val {
        Parameter::Register(register2) => return generate_st_reg_reg(register, &register2, opcode),
        Parameter::Indirect(parameter) => {
            return generate_st_reg_indirect(
                Spanned::new(parameter, operation.parameters[1].span),
                opcode,
                symbol_table,
            )
        }

        _ => {
            return Err(Error::new(
                "Expected either an indirect or register",
                operation.parameters[1].span,
            ))
        }
    }
}

fn generate_st_reg_reg(
    register: &Register,
    register2: &Register,
    mut opcode: u32,
) -> Result<u32, Error> {
    // MOVR
    opcode |= Mnemonic::PASSA.generate();
    opcode &= !(0b1111 << 16); // need to zero out previously set register
    opcode |= register2.generate() << 12;
    opcode |= register.generate() << 8;
    return Ok(opcode);
}

fn generate_st_reg_indirect(
    parameter: Spanned<&Parameter>,
    opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match parameter.val {
        Parameter::Register(register2) => return generate_st_reg_ireg(register2, opcode),
        Parameter::Number(number) => return generate_st_reg_inum(*number, opcode),
        Parameter::Label(label) => {
            return generate_st_reg_ilabel(
                Spanned::new(label, parameter.span),
                opcode,
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Nested indirection is not supported",
                parameter.span,
            ))
        }
    }
}

fn generate_st_reg_ireg(register2: &Register, mut opcode: u32) -> Result<u32, Error> {
    // STR
    opcode |= Mnemonic::STR.generate();
    opcode |= register2.generate() << 12;
    return Ok(opcode);
}

fn generate_st_reg_inum(number: u32, mut opcode: u32) -> Result<u32, Error> {
    // ST
    opcode |= Mnemonic::ST.generate();
    opcode |= number & 0xffff;
    return Ok(opcode);
}

fn generate_st_reg_ilabel(
    label: Spanned<&str>,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    // ST
    opcode |= Mnemonic::ST.generate();
    if let Some(label_line) = symbol_table.get(label.val) {
        opcode |= label_line & 0xffff;
        return Ok(opcode);
    } else {
        return Err(Error::new("Could not find label", label.span));
    }
}
