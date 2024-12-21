use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub fn generate_st(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.parameters.len() != 2 {
        return Err(Error::new(
            format!("Expected {} parameters", "2".fg(ATTENTION_COLOR)),
            operation.parameters.span,
        ));
    }

    match &operation.parameters[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => generate_st_reg(
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

fn generate_st_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register2) => {
            generate_st_reg_reg(modifiers, register, register2)
        }
        ExpressionResult::Indirect(parameter) => generate_st_reg_indirect(
            modifiers,
            register,
            &Spanned::new(parameter, parameters[1].span),
        ),

        _ => Err(Error::new(
            format!(
                "Expected either an {} or {}",
                "register".fg(ATTENTION_COLOR),
                "indirect".fg(ATTENTION_COLOR)
            ),
            parameters[1].span,
        )),
    }
}

fn generate_st_reg_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    // MOVR
    let mut opcode = 0;
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= Mnemonic::Pass.generate();
    opcode |= register2.generate() << 12;
    opcode |= register1.generate() << 4;
    Ok(opcode)
}

fn generate_st_reg_indirect(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameter: &Spanned<&ExpressionResult>,
) -> Result<u32, Error> {
    match parameter.val {
        ExpressionResult::Register(register2) => {
            generate_st_reg_ireg(modifiers, register, register2)
        }
        ExpressionResult::Number(number) => {
            generate_st_reg_inum(modifiers, register, &Spanned::new(**number, parameter.span))
        }
        ExpressionResult::RegisterOffset(reg_offset) => generate_st_reg_ireg_offset(
            modifiers,
            register,
            &reg_offset.reg,
            &Spanned::new(reg_offset.offset as i32, parameter.span),
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

fn generate_st_reg_ireg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::Str.generate();
    opcode |= register1.generate() << 16;
    opcode |= register2.generate() << 12;
    Ok(opcode)
}

fn generate_st_reg_ireg_offset(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    offset: &Spanned<i32>,
) -> Result<u32, Error> {
    assert_range(offset, (-1 << 11)..(1 << 11))?;
    let mut opcode = 0;
    opcode |= generate_st_reg_ireg(modifiers, register1, register2)?;
    opcode |= offset.val as u32 & 0xfff;
    Ok(opcode)
}

fn generate_st_reg_inum(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<u32>,
) -> Result<u32, Error> {
    assert_range(number, 0..(1 << 16))?;
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::St.generate();
    opcode |= register.generate() << 16;
    opcode |= number.val & 0xffff;
    Ok(opcode)
}
