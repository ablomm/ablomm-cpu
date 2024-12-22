use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub mod unary_alu_op;

pub fn generate_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.operands.len() == 2 {
        generate_alu_op_2(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.operands,
            symbol_table,
        )
    } else if operation.operands.len() == 3 {
        generate_alu_op_3(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.operands,
            symbol_table,
        )
    } else {
        Err(Error::new(
            format!(
                "Expected {} or {} operands",
                "2".fg(ATTENTION_COLOR),
                "3".fg(ATTENTION_COLOR)
            ),
            operation.operands.span,
        ))
    }
}

fn generate_alu_op_2(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg(mnemonic, modifiers, register, operands, symbol_table)
        }
        ExpressionResult::Number(number) => generate_alu_op_2_num(
            mnemonic,
            modifiers,
            &operands[0].span_to(**number),
            operands,
            symbol_table,
        ),
        _ => Err(Error::new(
            format!(
                "Expected a {} or {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[0].span,
        )),
    }
}

fn generate_alu_op_2_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register2) => {
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register2)
        }
        ExpressionResult::Number(number) => generate_alu_op_2_reg_num(
            mnemonic,
            modifiers,
            register,
            &operands[1].span_to(**number),
        ),
        _ => Err(Error::new(
            format!(
                "Expected a {} or {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[1].span,
        )),
    }
}

fn generate_alu_op_2_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    register2: &Register,
) -> Result<u32, Error> {
    generate_alu_op_3_reg_reg_reg(mnemonic, modifiers, register, register, register2)
}

fn generate_alu_op_2_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<u32>,
) -> Result<u32, Error> {
    generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register, register, number)
}

fn generate_alu_op_2_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    number: &Spanned<u32>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_num_reg(mnemonic, modifiers, number, register)
        }
        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[1].span,
        )),
    }
}

fn generate_alu_op_2_num_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    number: &Spanned<u32>,
    register: &Register,
) -> Result<u32, Error> {
    generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register)
}

fn generate_alu_op_3(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_3_reg(mnemonic, modifiers, register, operands, symbol_table)
        }
        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[0].span,
        )),
    }
}

fn generate_alu_op_3_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register2) => generate_alu_op_3_reg_reg(
            mnemonic,
            modifiers,
            register,
            register2,
            operands,
            symbol_table,
        ),
        ExpressionResult::Number(number) => generate_alu_op_3_reg_num(
            mnemonic,
            modifiers,
            register,
            &operands[1].span_to(**number),
            operands,
            symbol_table,
        ),
        _ => Err(Error::new(
            format!(
                "Expected a {} or {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[1].span,
        )),
    }
}

fn generate_alu_op_3_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[2].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register3) => {
            generate_alu_op_3_reg_reg_reg(mnemonic, modifiers, register1, register2, register3)
        }
        ExpressionResult::Number(number) => generate_alu_op_3_reg_reg_num(
            mnemonic,
            modifiers,
            register1,
            register2,
            &operands[2].span_to(**number),
        ),
        _ => Err(Error::new(
            format!(
                "Expected a {} or {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[2].span,
        )),
    }
}

fn generate_alu_op_3_reg_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    register3: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= register3.generate() << 4;
    Ok(opcode)
}

fn generate_alu_op_3_reg_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    number: &Spanned<u32>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    assert_range(number, 0..(1 << 8))?;
    opcode |= mnemonic.generate();
    opcode |= generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= number.val & 0xff;
    Ok(opcode)
}

fn generate_alu_op_3_reg_num(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    number: &Spanned<u32>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[2].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register2) => {
            generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register2)
        }
        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.val.fg(ATTENTION_COLOR)
            ),
            operands[2].span,
        )),
    }
}

fn generate_alu_op_3_reg_num_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    number: &Spanned<u32>,
    register2: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register1, register2, number)?;
    opcode |= AluOpFlags::Reverse.generate();
    Ok(opcode)
}
