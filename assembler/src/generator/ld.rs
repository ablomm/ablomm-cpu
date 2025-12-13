use crate::{
    ast::{AsmMnemonic, CpuMnemonic, Expression, Modifier, Operation, Register},
    error::Error,
    expression::expression_result::{ExpressionResult, Number, RegisterOffset},
    generator::{self, Generatable},
    span::Spanned,
    symbol_table::SymbolTable,
};

pub(super) fn generate_ld(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Ld
    ));

    if operation.operands.len() != 2 {
        return Err(Error::incorrect_num(
            operation.operands.span,
            "operand",
            vec![2],
            operation.operands.len(),
        ));
    }

    let operand =
        operation.operands[0].span_to(operation.operands[0].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            generate_ld_reg(
                &operation.full_mnemonic.modifiers.as_ref(),
                &register,
                &operation.operands.as_ref(),
                symbol_table,
            )
        }
        ExpressionResult::Indirect(indirect) => generate_ld_indirect(
            &operation.full_mnemonic.modifiers.as_ref(),
            &operand.span_to(indirect),
            &operation.operands.as_ref(),
            symbol_table,
        ),
        _ => Err(Error::incorrect_type(
            vec!["register", "indirect"],
            &operand.as_ref(),
        )),
    }
}

fn generate_ld_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Number(number) => {
            let number = operand.span_to(number).unwrap();
            generate_ld_reg_num(modifiers, register, &number)
        }
        ExpressionResult::Register(register2) => {
            let register2 = operand.span_to(register2).unwrap();
            generate_ld_reg_reg(modifiers, register, &register2)
        }
        ExpressionResult::Indirect(indirect) => {
            generate_ld_reg_indirect(modifiers, register, &operand.span_to(indirect))
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register", "indirect"],
            &operand.as_ref(),
        )),
    }
}

// MOV
fn generate_ld_reg_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generator::generate_modifiers_alu(modifiers)?;
    opcode |= CpuMnemonic::Pass.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 4;
    Ok(opcode)
}

fn generate_ld_reg_num(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    number: &Spanned<&Number>,
) -> Result<u32, Error> {
    generator::assert_range(&number.as_u32().copied(), 0..(1 << 16))?;

    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Ldi.generate();
    opcode |= register.generate() << 16;
    opcode |= number.as_u32().val & 0xffff;
    Ok(opcode)
}

fn generate_ld_reg_indirect(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    indirect: &Spanned<&ExpressionResult>,
) -> Result<u32, Error> {
    match indirect.val {
        ExpressionResult::Number(number) => {
            let number = indirect.span_to(number).unwrap();
            generate_ld_reg_inum(modifiers, register, &number)
        }
        ExpressionResult::Register(register2) => {
            let register2 = indirect.span_to(register2).unwrap();
            generate_ld_reg_ireg(modifiers, register, &register2)
        }
        ExpressionResult::RegisterOffset(reg_offset) => {
            let reg_offset = indirect.span_to(reg_offset).unwrap();
            generate_ld_reg_ireg_offset(modifiers, register, &reg_offset)
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register", "register offset"],
            indirect,
        )),
    }
}

fn generate_ld_reg_ireg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Ldr.generate();
    opcode |= register1.generate() << 16;
    opcode |= register2.generate() << 12;
    Ok(opcode)
}

fn generate_ld_reg_ireg_offset(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    reg_offset: &Spanned<&RegisterOffset>,
) -> Result<u32, Error> {
    generator::assert_range(
        &reg_offset.span_to(reg_offset.offset),
        (-1 << 11)..(1 << 11),
    )?;

    let mut opcode = 0;
    opcode |= generate_ld_reg_ireg(modifiers, register, &reg_offset.reg.as_ref())?;
    opcode |= reg_offset.offset as u32 & 0xfff;
    Ok(opcode)
}

fn generate_ld_reg_inum(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    number: &Spanned<&Number>,
) -> Result<u32, Error> {
    generator::assert_range(&number.as_u32().copied(), 0..(1 << 16))?;

    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Ld.generate();
    opcode |= register.generate() << 16;
    opcode |= number.as_u32().val & 0xffff;
    Ok(opcode)
}

fn generate_ld_indirect(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    indirect: &Spanned<&ExpressionResult>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match indirect.val {
        ExpressionResult::Number(number) => {
            let number = indirect.span_to(number).unwrap();
            generate_ld_inum(modifiers, &number, operands, symbol_table)
        }
        ExpressionResult::Register(register) => {
            let register = indirect.span_to(register).unwrap();
            generate_ld_ireg(modifiers, &register, operands, symbol_table)
        }
        ExpressionResult::RegisterOffset(reg_offset) => {
            let reg_offset = indirect.span_to(reg_offset).unwrap();
            generate_ld_ireg_offset(modifiers, &reg_offset, operands, symbol_table)
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register", "register offset"],
            indirect,
        )),
    }
}

fn generate_ld_ireg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register2) => {
            let register2 = operand.span_to(register2).unwrap();
            generate_ld_ireg_reg(modifiers, register, &register2)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_ld_ireg_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
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
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            generate_ld_ireg_offset_reg(modifiers, reg_offset, &register)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_ld_ireg_offset_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    reg_offset: &Spanned<&RegisterOffset>,
    register: &Spanned<&Register>,
) -> Result<u32, Error> {
    generator::assert_range(
        &reg_offset.span_to(reg_offset.offset),
        (-1 << 11)..(1 << 11),
    )?;

    let mut opcode = 0;
    opcode |= generate_ld_ireg_reg(modifiers, &reg_offset.reg.as_ref(), register)?;
    opcode |= reg_offset.offset as u32 & 0xfff;
    Ok(opcode)
}

fn generate_ld_inum(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    number: &Spanned<&Number>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            generate_ld_inum_reg(modifiers, number, &register)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_ld_inum_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    number: &Spanned<&Number>,
    register: &Spanned<&Register>,
) -> Result<u32, Error> {
    generator::assert_range(&number.as_u32().copied(), 0..(1 << 16))?;

    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::St.generate();
    opcode |= register.generate() << 16;
    opcode |= number.as_u32().val & 0xffff;
    Ok(opcode)
}
