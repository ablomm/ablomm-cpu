use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub mod unary_alu_op;

pub fn generate_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.parameters.len() == 2 {
        generate_alu_op_2(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        )
    } else if operation.parameters.len() == 3 {
        generate_alu_op_3(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        )
    } else {
        Err(Error::new(
            format!(
                "Expected {} or {} parameters",
                "2".fg(ATTENTION_COLOR),
                "3".fg(ATTENTION_COLOR)
            ),
            operation.parameters.span,
        ))
    }
}

// parameter length 2
fn generate_alu_op_2(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        ExpressionResult::Number(number) => generate_alu_op_2_num(
            mnemonic,
            modifiers,
            &Spanned::new(**number, parameters[0].span),
            parameters,
            symbol_table,
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {} or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR)
            ),
            parameters[0].span,
        )),
    }
}

// when first parameter is a register
fn generate_alu_op_2_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register2) => {
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register2)
        }
        ExpressionResult::Number(number) => generate_alu_op_2_reg_num(
            mnemonic,
            modifiers,
            register,
            &Spanned::new(**number, parameters[1].span),
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {} or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR)
            ),
            parameters[1].span,
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
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_num_reg(mnemonic, modifiers, number, register)
        }
        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            parameters[1].span,
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

// parameter length 3
fn generate_alu_op_3(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_3_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            parameters[0].span,
        )),
    }
}

fn generate_alu_op_3_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[1].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register2) => generate_alu_op_3_reg_reg(
            mnemonic,
            modifiers,
            register,
            register2,
            parameters,
            symbol_table,
        ),
        ExpressionResult::Number(number) => generate_alu_op_3_reg_num(
            mnemonic,
            modifiers,
            register,
            &Spanned::new(**number, parameters[1].span),
            parameters,
            symbol_table,
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {} or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR)
            ),
            parameters[1].span,
        )),
    }
}

fn generate_alu_op_3_reg_reg(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register1: &Register,
    register2: &Register,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[2].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register3) => {
            generate_alu_op_3_reg_reg_reg(mnemonic, modifiers, register1, register2, register3)
        }
        ExpressionResult::Number(number) => generate_alu_op_3_reg_reg_num(
            mnemonic,
            modifiers,
            register1,
            register2,
            &Spanned::new(**number, parameters[2].span),
        ),
        _ => Err(Error::new(
            format!(
                "Expected either a {} or {}",
                "register".fg(ATTENTION_COLOR),
                "expression".fg(ATTENTION_COLOR)
            ),
            parameters[2].span,
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
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[2].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register2) => {
            generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register2)
        }
        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            parameters[2].span,
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
