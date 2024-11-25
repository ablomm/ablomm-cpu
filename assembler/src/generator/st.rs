use crate::generator::*;

pub fn generate_st(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.parameters.len() != 2 {
        return Err(Error::new(
            "Expected 2 parameters",
            operation.parameters.span,
        ));
    }

    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            return generate_st_reg(
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

fn generate_st_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register2) => {
            return generate_st_reg_reg(modifiers, register, &register2)
        }
        Parameter::Indirect(parameter) => {
            return generate_st_reg_indirect(
                modifiers,
                register,
                &Spanned::new(parameter, parameters[1].span),
                symbol_table,
            )
        }

        _ => {
            return Err(Error::new(
                "Expected either an indirect or register",
                parameters[1].span,
            ))
        }
    }
}

fn generate_st_reg_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    // MOVR
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= Mnemonic::PASS.generate();
    opcode |= register2.generate() << 12;
    opcode |= register1.generate() << 4;
    return Ok(opcode);
}

fn generate_st_reg_indirect(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameter: &Spanned<&Parameter>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match parameter.val {
        Parameter::Register(register2) => {
            return generate_st_reg_ireg(modifiers, register, register2)
        }
        Parameter::Expression(expression) => {
            return generate_st_reg_inum(
                modifiers,
                register,
                expression.eval(parameter.span, symbol_table)?,
            )
        }
        Parameter::RegisterOffset(register2, offset) => {
            return generate_st_reg_ireg_offset(
                modifiers,
                register,
                register2,
                offset.eval(parameter.span, symbol_table)?,
            )
        }
        _ => {
            return Err(Error::new(
                "Expected either a register, expression, or register offset",
                parameter.span,
            ))
        }
    }
}

fn generate_st_reg_ireg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    return generate_st_reg_ireg_offset(modifiers, register1, register2, 0);
}

fn generate_st_reg_ireg_offset(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    offset: i64,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::STR.generate();
    opcode |= register1.generate() << 16;
    opcode |= register2.generate() << 12;
    opcode |= offset as u32 & 0xfff;
    return Ok(opcode);
}

fn generate_st_reg_inum(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: i64,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::ST.generate();
    opcode |= register.generate() << 16;
    opcode |= number as u32 & 0xffff;
    return Ok(opcode);
}
