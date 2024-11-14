use crate::generator::*;
use std::collections::HashMap;

pub mod unary_alu_op;

pub fn generate_alu_op(
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    if operation.parameters.len() == 2 {
        return generate_alu_op_2(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        );
    } else if operation.parameters.len() == 3 {
        return generate_alu_op_3(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        );
    } else {
        return Err(Error::new(
            "Expected 2 or 3 parameters",
            operation.parameters.span,
        ));
    }
}

// parameter length 2
fn generate_alu_op_2(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_2_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        Parameter::Number(number) => {
            return generate_alu_op_2_num(mnemonic, modifiers, *number, parameters)
        }
        _ => {
            return Err(Error::new(
                "Expected either a register or number",
                parameters[0].span,
            ))
        }
    }
}

// when first parameter is a register
fn generate_alu_op_2_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register2) => {
            return generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register2)
        }
        Parameter::Number(number) => {
            return generate_alu_op_2_reg_num(mnemonic, modifiers, register, *number)
        }
        Parameter::Label(label) => {
            return generate_alu_op_2_reg_label(
                mnemonic,
                modifiers,
                register,
                &Spanned::new(label, parameters[1].span),
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, number, or label",
                parameters[1].span,
            ))
        }
    }
}

fn generate_alu_op_2_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    return generate_alu_op_3_reg_reg_reg(mnemonic, modifiers, register, register, register2);
}

fn generate_alu_op_2_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: u32,
) -> Result<u32, Error> {
    return generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register, register, number);
}

fn generate_alu_op_2_reg_label(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    label: &Spanned<&str>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    return generate_alu_op_3_reg_reg_label(
        mnemonic,
        modifiers,
        register,
        register,
        label,
        symbol_table,
    );
}

fn generate_alu_op_2_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    number: u32,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register) => {
            return generate_alu_op_2_num_reg(mnemonic, modifiers, number, register)
        }
        _ => return Err(Error::new("Expected a register", parameters[1].span)),
    }
}

fn generate_alu_op_2_num_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    number: u32,
    register: &Register,
) -> Result<u32, Error> {
    return generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register);
}

// parameter length 3
fn generate_alu_op_3(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_3_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        _ => return Err(Error::new("Expected a register", parameters[0].span)),
    }
}

fn generate_alu_op_3_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register2) => {
            return generate_alu_op_3_reg_reg(
                mnemonic,
                modifiers,
                register,
                register2,
                parameters,
                symbol_table,
            )
        }
        Parameter::Number(number) => {
            return generate_alu_op_3_reg_num(mnemonic, modifiers, register, *number, parameters)
        }
        Parameter::Label(label) => {
            return generate_alu_op_3_reg_label(
                mnemonic,
                modifiers,
                register,
                &Spanned::new(label, parameters[1].span),
                parameters,
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, number, or label",
                parameters[1].span,
            ))
        }
    }
}

fn generate_alu_op_3_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[2].val {
        Parameter::Register(register3) => {
            return generate_alu_op_3_reg_reg_reg(
                mnemonic, modifiers, register1, register2, register3,
            )
        }
        Parameter::Number(number) => {
            return generate_alu_op_3_reg_reg_num(
                mnemonic, modifiers, register1, register2, *number,
            )
        }
        Parameter::Label(label) => {
            return generate_alu_op_3_reg_reg_label(
                mnemonic,
                modifiers,
                register1,
                register2,
                &Spanned::new(label, parameters[2].span),
                symbol_table,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, number, or label",
                parameters[2].span,
            ))
        }
    }
}

fn generate_alu_op_3_reg_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    register3: &Register,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= register3.generate() << 4;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    number: u32,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= number & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_reg_label(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    label: &Spanned<&str>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= get_label_address(label, symbol_table)? & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: u32,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
) -> Result<u32, Error> {
    match &parameters[2].val {
        Parameter::Register(register2) => {
            return generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register2)
        }
        _ => return Err(Error::new("Expected a register", parameters[2].span)),
    }
}

fn generate_alu_op_3_reg_num_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    number: u32,
    register2: &Register,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= number & 0xff;
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_label(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    label: &Spanned<&str>,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[2].val {
        Parameter::Register(register2) => generate_alu_op_3_reg_label_reg(
            mnemonic,
            modifiers,
            register,
            label,
            register2,
            symbol_table,
        ),
        _ => return Err(Error::new("Expected a register", label.span)),
    }
}

fn generate_alu_op_3_reg_label_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    label: &Spanned<&str>,
    register2: &Register,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Reverse.generate();
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= get_label_address(label, symbol_table)? & 0xff;
    opcode |= register2.generate() << 8;
    return Ok(opcode);
}
