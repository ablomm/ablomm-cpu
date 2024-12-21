use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub fn generate_ld(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.operands.len() != 2 {
        return Err(Error::new(
            format!("Expected {} operands", "2".fg(ATTENTION_COLOR)),
            operation.operands.span,
        ));
    }

    match &operation.operands[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => generate_ld_reg(
            &operation.full_mnemonic.modifiers,
            register,
            &operation.operands,
            symbol_table,
        ),

        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            operation.operands[0].span,
        )),
    }
}

fn generate_ld_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &operands[1].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register2) => {
            generate_ld_reg_reg(modifiers, register, register2)
        }

        ExpressionResult::Number(number) => generate_ld_reg_num(
            modifiers,
            register,
            &Spanned::new(**number, operands[1].span),
        ),
        ExpressionResult::Indirect(operand) => generate_ld_reg_indirect(
            modifiers,
            register,
            &Spanned::new(operand, operands[1].span),
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {}, {}, or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                "indirect".fg(ATTENTION_COLOR)
            ),
            operands[1].span,
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
    operand: &Spanned<&ExpressionResult>,
) -> Result<u32, Error> {
    match operand.val {
        ExpressionResult::Register(register2) => {
            generate_ld_reg_ireg(modifiers, register, register2)
        }
        ExpressionResult::Number(number) => {
            generate_ld_reg_inum(modifiers, register, &Spanned::new(**number, operand.span))
        }
        ExpressionResult::RegisterOffset(reg_offset) => generate_ld_reg_ireg_offset(
            modifiers,
            register,
            &reg_offset.reg,
            &Spanned::new(reg_offset.offset as i32, operand.span),
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {}, {}, or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                "register offset".fg(ATTENTION_COLOR)
            ),
            operand.span,
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
