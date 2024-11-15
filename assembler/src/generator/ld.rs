use crate::generator::*;
use std::collections::HashMap;

pub fn generate_ld(
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    if operation.parameters.len() != 2 {
        return Err(Error::new(
            "Expected 2 parameters",
            operation.parameters.span,
        ));
    }

    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_ld_reg(
                &operation.full_mnemonic.modifiers,
                register,
                &operation.parameters,
                symbol_table,
            )
        }

        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_ld_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register2) => {
            return generate_ld_reg_reg(modifiers, register, &register2)
        }

        Parameter::Number(number) => return generate_ld_reg_num(modifiers, register, *number),
        Parameter::Label(label) => {
            return generate_ld_reg_label(
                modifiers,
                register,
                &Spanned::new(label, parameters[1].span),
                symbol_table,
            )
        }
        Parameter::Indirect(parameter) => {
            return generate_ld_reg_indirect(
                modifiers,
                register,
                &Spanned::new(parameter, parameters[1].span),
                symbol_table,
            )
        }
    }
}

fn generate_ld_reg_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    // MOV
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= Mnemonic::PASSA.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}

fn generate_ld_reg_num(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: u32,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::LDI.generate();
    opcode |= register.generate() << 16;
    opcode |= number & 0xffff;
    return Ok(opcode);
}

fn generate_ld_reg_label(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    label: &Spanned<&str>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    return generate_ld_reg_num(modifiers, register, get_label_address(label, symbol_table)?);
}

fn generate_ld_reg_indirect(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameter: &Spanned<&Parameter>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match parameter.val {
        Parameter::Register(register2) => {
            return generate_ld_reg_ireg(modifiers, register, register2)
        }
        Parameter::Number(number) => return generate_ld_reg_inum(modifiers, register, *number),
        Parameter::Label(label) => {
            return generate_ld_reg_ilabel(
                modifiers,
                register,
                &Spanned::new(label, parameter.span),
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

// indirect register
fn generate_ld_reg_ireg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::LDR.generate();
    opcode |= register1.generate() << 16;
    opcode |= register2.generate() << 12;
    return Ok(opcode);
}

fn generate_ld_reg_inum(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: u32,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::LD.generate();
    opcode |= register.generate() << 16;
    opcode |= number & 0xffff;
    return Ok(opcode);
}

fn generate_ld_reg_ilabel(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    label: &Spanned<&str>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    return generate_ld_reg_inum(modifiers, register, get_label_address(label, symbol_table)?);
}
