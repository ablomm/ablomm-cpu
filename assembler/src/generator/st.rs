use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub fn generate_st(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.operands.len() != 2 {
        return Err(Error::new(
            format!("Expected {} operands", "2".fg(ATTENTION_COLOR)),
            operation.operands.span,
        ));
    }

    let operand = operation.operands[0].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register) => generate_st_reg(
            &operation.full_mnemonic.modifiers,
            register,
            &operation.operands,
            symbol_table,
        ),

        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operation.operands[0].span,
        )),
    }
}

fn generate_st_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register2) => {
            generate_st_reg_reg(modifiers, register, register2)
        }
        ExpressionResult::Indirect(operand) => {
            generate_st_reg_indirect(modifiers, register, &operands[1].span_to(operand))
        }

        _ => Err(Error::new(
            format!(
                "Expected a {} or {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                "indirect".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR),
            ),
            operands[1].span,
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
    operand: &Spanned<&ExpressionResult>,
) -> Result<u32, Error> {
    match operand.val {
        ExpressionResult::Register(register2) => {
            generate_st_reg_ireg(modifiers, register, register2)
        }
        ExpressionResult::Number(number) => {
            generate_st_reg_inum(modifiers, register, &operand.span_to(**number))
        }
        ExpressionResult::RegisterOffset(reg_offset) => generate_st_reg_ireg_offset(
            modifiers,
            register,
            &reg_offset.reg,
            &operand.span_to(reg_offset.offset as i32),
        ),
        _ => Err(Error::new(
            format!(
                "Expected a {}, {}, or {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                "register offset".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operand.span,
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
