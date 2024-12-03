use ariadne::Fmt;

use crate::generator::*;

pub fn generate_ld(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.parameters.len() != 2 {
        return Err(Error::new(
            format!("Expected {} parameters", "2".fg(ATTENTION_COLOR)),
            operation.parameters.span,
        ));
    }

    match &operation.parameters[0].val {
        Parameter::Register(register) => generate_ld_reg(
            &operation.full_mnemonic.modifiers,
            register,
            &operation.parameters,
            symbol_table,
        ),

        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            operation.parameters[0].span,
        )),
    }
}

fn generate_ld_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].val {
        Parameter::Register(register2) => generate_ld_reg_reg(modifiers, register, register2),

        Parameter::Expression(expression) => generate_ld_reg_num(
            modifiers,
            register,
            &Spanned::new(
                Spanned::new(expression, parameters[1].span).eval(symbol_table)?,
                parameters[1].span,
            ),
        ),
        Parameter::Indirect(parameter) => generate_ld_reg_indirect(
            modifiers,
            register,
            &Spanned::new(parameter, parameters[1].span),
            symbol_table,
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {}, {}, or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                "indirect".fg(ATTENTION_COLOR)
            ),
            parameters[1].span,
        )),
    }
}

fn generate_ld_reg_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    // MOV
    let mut opcode = 0;
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= Mnemonic::Pass.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 4;
    Ok(opcode)
}

fn generate_ld_reg_num(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<u32>,
) -> Result<u32, Error> {
    assert_range(number, 0..(1 << 16))?;
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::Ldi.generate();
    opcode |= register.generate() << 16;
    opcode |= number.val & 0xffff;
    Ok(opcode)
}

fn generate_ld_reg_indirect(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameter: &Spanned<&Parameter>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match parameter.val {
        Parameter::Register(register2) => generate_ld_reg_ireg(modifiers, register, register2),
        Parameter::Expression(expression) => generate_ld_reg_inum(
            modifiers,
            register,
            &Spanned::new(
                Spanned::new(expression, parameter.span).eval(symbol_table)?,
                parameter.span,
            ),
        ),
        Parameter::RegisterOffset(register2, offset) => generate_ld_reg_ireg_offset(
            modifiers,
            register,
            register2,
            &Spanned::new(offset.as_ref().eval(symbol_table)? as i32, offset.span),
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {}, {}, or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                "register offset".fg(ATTENTION_COLOR)
            ),
            parameter.span,
        )),
    }
}

// indirect register
fn generate_ld_reg_ireg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::Ldr.generate();
    opcode |= register1.generate() << 16;
    opcode |= register2.generate() << 12;
    Ok(opcode)
}

fn generate_ld_reg_ireg_offset(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    offset: &Spanned<i32>,
) -> Result<u32, Error> {
    assert_range(offset, (-1 << 11)..(1 << 11))?;
    let mut opcode = 0;
    opcode |= generate_ld_reg_ireg(modifiers, register1, register2)?;
    opcode |= offset.val as u32 & 0xfff;
    Ok(opcode)
}

fn generate_ld_reg_inum(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<u32>,
) -> Result<u32, Error> {
    assert_range(number, 0..(1 << 16))?;
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::Ld.generate();
    opcode |= register.generate() << 16;
    opcode |= number.val & 0xffff;
    Ok(opcode)
}
