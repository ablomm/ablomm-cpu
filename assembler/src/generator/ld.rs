use crate::error::*;
use crate::generator::Generatable;
use crate::parser::*;
use crate::span::*;
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
    let mut opcode: u32 = 0;
    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);

    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_ld_reg(register, opcode, operation, symbol_table)
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
    register: &Register,
    mut opcode: u32,
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    opcode |= register.generate() << 16;

    match &operation.parameters[1].val {
        Parameter::Register(register2) => return generate_ld_reg_reg(register, &register2, opcode),

        Parameter::Number(number) => return generate_ld_reg_num(*number, opcode),
        Parameter::Label(label) => {
            return generate_ld_reg_label(label, operation.parameters[1].span, opcode, symbol_table)
        }
        Parameter::Indirect(parameter) => {
            return generate_ld_reg_indirect(
                parameter,
                operation.parameters[1].span,
                opcode,
                symbol_table,
            )
        }
    }
}

fn generate_ld_reg_reg(
    register: &Register,
    register2: &Register,
    mut opcode: u32,
) -> Result<u32, Error> {
    // MOV
    // this is really just an alu_op
    opcode |= Mnemonic::PASSA.generate();
    opcode &= !(0b1111 << 16); // need to zero out previously set register
    opcode |= register.generate() << 12;
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}

fn generate_ld_reg_num(number: u32, mut opcode: u32) -> Result<u32, Error> {
    // LDI
    opcode |= Mnemonic::LDI.generate();
    opcode |= number & 0xffff;
    return Ok(opcode);
}

fn generate_ld_reg_label(
    label: &str,
    span: Span,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    // LDI
    opcode |= Mnemonic::LDI.generate();
    if let Some(label_line) = symbol_table.get(label) {
        opcode |= label_line & 0xffff;
        return Ok(opcode);
    } else {
        return Err(Error::new("Could not find label", span));
    }
}

fn generate_ld_reg_indirect(
    parameter: &Parameter,
    span: Span,
    opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match parameter {
        Parameter::Register(register2) => return generate_ld_reg_ireg(register2, opcode),
        Parameter::Number(number) => return generate_ld_reg_inum(*number, opcode),
        Parameter::Label(label) => {
            return generate_ld_reg_ilabel(label, span, opcode, symbol_table)
        }
        _ => return Err(Error::new("Nested indirection is not supported", span)),
    }
}
// indirect register
fn generate_ld_reg_ireg(register2: &Register, mut opcode: u32) -> Result<u32, Error> {
    // LDR
    opcode |= Mnemonic::LDR.generate();
    opcode |= register2.generate() << 12;
    return Ok(opcode);
}

fn generate_ld_reg_inum(number: u32, mut opcode: u32) -> Result<u32, Error> {
    // LD
    opcode |= Mnemonic::LD.generate();
    opcode |= number & 0xffff;
    return Ok(opcode);
}

fn generate_ld_reg_ilabel(
    label: &str,
    span: Span,
    mut opcode: u32,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    // LD
    opcode |= Mnemonic::LD.generate();
    if let Some(label_line) = symbol_table.get(label) {
        opcode |= label_line & 0xffff;
        return Ok(opcode);
    } else {
        return Err(Error::new("Could not find label", span));
    }
}
