use crate::generator::*;

pub mod unary_alu_op;

pub fn generate_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
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
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_2_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        Parameter::Expression(expression) => {
            return generate_alu_op_2_num(
                mnemonic,
                modifiers,
                expression.eval(parameters[0].span, symbol_table)?,
                parameters,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register or expression",
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
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register2) => {
            return generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register2)
        }
        Parameter::Expression(expression) => {
            return generate_alu_op_2_reg_num(
                mnemonic,
                modifiers,
                register,
                expression.eval(parameters[1].span, symbol_table)?,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register or expression",
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
    number: i64,
) -> Result<u32, Error> {
    return generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register, register, number);
}

fn generate_alu_op_2_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    number: i64,
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
    number: i64,
    register: &Register,
) -> Result<u32, Error> {
    return generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register);
}

// parameter length 3
fn generate_alu_op_3(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &SymbolTable,
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
    symbol_table: &SymbolTable,
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
        Parameter::Expression(expression) => {
            return generate_alu_op_3_reg_num(
                mnemonic,
                modifiers,
                register,
                expression.eval(parameters[1].span, symbol_table)?,
                parameters,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register or expression",
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
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[2].val {
        Parameter::Register(register3) => {
            return generate_alu_op_3_reg_reg_reg(
                mnemonic, modifiers, register1, register2, register3,
            )
        }
        Parameter::Expression(expression) => {
            return generate_alu_op_3_reg_reg_num(
                mnemonic,
                modifiers,
                register1,
                register2,
                expression.eval(parameters[2].span, symbol_table)?,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register or expression",
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
    number: i64,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= number as u32 & 0xff;
    return Ok(opcode);
}

fn generate_alu_op_3_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: i64,
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
    number: i64,
    register2: &Register,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register1, register2, number)?;
    opcode |= AluOpFlags::Reverse.generate();
    return Ok(opcode);
}
