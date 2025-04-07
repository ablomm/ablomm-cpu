use crate::{
    expression::expression_result::{ExpressionResult, RegisterOffset},
    generator::*,
};

pub fn generate_ld(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Ld
    ));

    if operation.operands.len() != 2 {
        return Err(SpannedError::incorrect_num(
            operation.operands.span,
            "operand",
            vec![2],
            operation.operands.len(),
        ));
    }

    let operand = operation.operands[0].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register) => {
            let register = &register.expect("Expression resulted in None while generating");
            generate_ld_reg(
                &operation.full_mnemonic.modifiers.as_ref(),
                register,
                &operation.operands.as_ref(),
                symbol_table,
            )
        }
        ExpressionResult::Indirect(indirect) => generate_ld_indirect(
            &operation.full_mnemonic.modifiers.as_ref(),
            &operation.operands[0].span_to(indirect),
            &operation.operands.as_ref(),
            symbol_table,
        ),
        _ => Err(SpannedError::incorrect_value(
            operation.operands[0].span,
            "type",
            vec!["register", "indirect"],
            Some(operand),
        )),
    }
}

fn generate_ld_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Register,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[1].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Number(number) => {
            let number = &number.expect("Expression resulted in None while generating");
            generate_ld_reg_num(modifiers, register, &operands[1].span_to(&**number))
        }
        ExpressionResult::Register(register2) => {
            let register2 = &register2.expect("Expression resulted in None while generating");
            generate_ld_reg_reg(modifiers, register, register2)
        }
        ExpressionResult::Indirect(indirect) => {
            generate_ld_reg_indirect(modifiers, register, &operands[1].span_to(indirect))
        }
        _ => Err(SpannedError::incorrect_value(
            operands[1].span,
            "type",
            vec!["number", "register", "indirect"],
            Some(operand),
        )),
    }
}

fn generate_ld_reg_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, SpannedError> {
    // MOV
    let mut opcode = 0;
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= CpuMnemonic::Pass.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 4;
    Ok(opcode)
}

fn generate_ld_reg_num(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<&u32>,
) -> Result<u32, SpannedError> {
    assert_range(&number.copied(), 0..(1 << 16))?;
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Ldi.generate();
    opcode |= register.generate() << 16;
    opcode |= number.val & 0xffff;
    Ok(opcode)
}

fn generate_ld_reg_indirect(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Register,
    indirect: &Spanned<&ExpressionResult>,
) -> Result<u32, SpannedError> {
    match indirect.val {
        ExpressionResult::Number(number) => {
            let number = &number.expect("Expression resulted in None while generating");
            generate_ld_reg_inum(modifiers, register, &indirect.span_to(&**number))
        }
        ExpressionResult::Register(register2) => {
            let register2 = &register2.expect("Expression resulted in None while generating");
            generate_ld_reg_ireg(modifiers, register, register2)
        }
        ExpressionResult::RegisterOffset(reg_offset) => {
            let reg_offset = &reg_offset.expect("Expression resulted in None while generating");
            generate_ld_reg_ireg_offset(modifiers, register, &indirect.span_to(reg_offset))
        }
        _ => Err(SpannedError::incorrect_value(
            indirect.span,
            "type",
            vec!["number", "register", "register offset"],
            Some(indirect.val),
        )),
    }
}

fn generate_ld_reg_ireg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, SpannedError> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Ldr.generate();
    opcode |= register1.generate() << 16;
    opcode |= register2.generate() << 12;
    Ok(opcode)
}

fn generate_ld_reg_ireg_offset(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Register,
    reg_offset: &Spanned<&RegisterOffset>,
) -> Result<u32, SpannedError> {
    assert_range(
        &reg_offset.span_to(reg_offset.offset),
        (-1 << 11)..(1 << 11),
    )?;
    let mut opcode = 0;
    opcode |= generate_ld_reg_ireg(modifiers, register, &reg_offset.reg)?;
    opcode |= reg_offset.offset as u32 & 0xfff;
    Ok(opcode)
}

fn generate_ld_reg_inum(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<&u32>,
) -> Result<u32, SpannedError> {
    assert_range(&number.copied(), 0..(1 << 16))?;
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Ld.generate();
    opcode |= register.generate() << 16;
    opcode |= number.val & 0xffff;
    Ok(opcode)
}

fn generate_ld_indirect(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    indirect: &Spanned<&ExpressionResult>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    match indirect.val {
        ExpressionResult::Number(number) => {
            let number = &number.expect("Expression resulted in None while generating");
            generate_ld_inum(
                modifiers,
                &indirect.span_to(&**number),
                operands,
                symbol_table,
            )
        }
        ExpressionResult::Register(register) => {
            let register = &register.expect("Expression resulted in None while generating");
            generate_ld_ireg(modifiers, register, operands, symbol_table)
        }
        ExpressionResult::RegisterOffset(reg_offset) => {
            let reg_offset = &reg_offset.expect("Expression resulted in None while generating");
            generate_ld_ireg_offset(
                modifiers,
                &indirect.span_to(reg_offset),
                operands,
                symbol_table,
            )
        }
        _ => Err(SpannedError::incorrect_value(
            indirect.span,
            "type",
            vec!["number", "register", "register offset"],
            Some(indirect.val),
        )),
    }
}

fn generate_ld_ireg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Register,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[1].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register2) => {
            let register2 = &register2.expect("Expression resulted in None while generating");
            generate_ld_ireg_reg(modifiers, register, register2)
        }
        _ => Err(SpannedError::incorrect_value(
            operands[1].span,
            "type",
            vec!["register"],
            Some(operand),
        )),
    }
}

fn generate_ld_ireg_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
) -> Result<u32, SpannedError> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Str.generate();
    opcode |= register2.generate() << 16;
    opcode |= register1.generate() << 12;
    Ok(opcode)
}

fn generate_ld_ireg_offset(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    reg_offset: &Spanned<&RegisterOffset>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[1].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register) => {
            let register = &register.expect("Expression resulted in None while generating");
            generate_ld_ireg_offset_reg(modifiers, reg_offset, register)
        }
        _ => Err(SpannedError::incorrect_value(
            operands[1].span,
            "type",
            vec!["register"],
            Some(operand),
        )),
    }
}

fn generate_ld_ireg_offset_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    reg_offset: &Spanned<&RegisterOffset>,
    register: &Register,
) -> Result<u32, SpannedError> {
    assert_range(
        &reg_offset.span_to(reg_offset.offset),
        (-1 << 11)..(1 << 11),
    )?;
    let mut opcode = 0;
    opcode |= generate_ld_ireg_reg(modifiers, &reg_offset.reg, register)?;
    opcode |= reg_offset.offset as u32 & 0xfff;
    Ok(opcode)
}

fn generate_ld_inum(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    number: &Spanned<&u32>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[1].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register) => {
            let register = &register.expect("Expression resulted in None while generating");
            generate_ld_inum_reg(modifiers, number, register)
        }
        _ => Err(SpannedError::incorrect_value(
            operands[1].span,
            "type",
            vec!["register"],
            Some(operand),
        )),
    }
}

fn generate_ld_inum_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    number: &Spanned<&u32>,
    register: &Register,
) -> Result<u32, SpannedError> {
    assert_range(&number.copied(), 0..(1 << 16))?;
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::St.generate();
    opcode |= register.generate() << 16;
    opcode |= number.val & 0xffff;
    Ok(opcode)
}
