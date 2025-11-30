use crate::{
    ast::{AluOpFlags, AsmMnemonic, CpuMnemonic, Expression, Modifier, Operation, Register},
    error::Error,
    expression::expression_result::{ExpressionResult, Number},
    generator::{self, Generatable},
    span::Spanned,
    symbol_table::SymbolTable,
};

mod unary_alu_op;

pub use unary_alu_op::generate_unary_alu_op;

pub fn generate_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let mnemonic = if let AsmMnemonic::BinaryAlu(mnemonic) = operation.full_mnemonic.mnemonic.val {
        operation.full_mnemonic.mnemonic.span_to(mnemonic)
    } else {
        panic!("Function was not called with AsmMnemonic::BinaryAlu");
    };

    if operation.operands.len() == 2 {
        generate_alu_op_2(
            &mnemonic.as_ref(),
            &operation.full_mnemonic.modifiers.as_ref(),
            &operation.operands.as_ref(),
            symbol_table,
        )
    } else if operation.operands.len() == 3 {
        generate_alu_op_3(
            &mnemonic.as_ref(),
            &operation.full_mnemonic.modifiers.as_ref(),
            &operation.operands.as_ref(),
            symbol_table,
        )
    } else {
        Err(Error::incorrect_num(
            operation.operands.span,
            "operand",
            vec![2, 3],
            operation.operands.len(),
        ))
    }
}

fn generate_alu_op_2(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].span_to(operands[0].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Number(number) => {
            let number = operand.span_to(number).unwrap();
            generate_alu_op_2_num(mnemonic, modifiers, &number, operands, symbol_table)
        }
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            generate_alu_op_2_reg(mnemonic, modifiers, &register, operands, symbol_table)
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register"],
            &operand.as_ref(),
        )),
    }
}

fn generate_alu_op_2_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Number(number) => {
            let number = operand.span_to(number).unwrap();
            generate_alu_op_2_reg_num(mnemonic, modifiers, register, &number)
        }
        ExpressionResult::Register(register2) => {
            let register2 = operand.span_to(register2).unwrap();
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, &register2)
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register"],
            &operand.as_ref(),
        )),
    }
}

fn generate_alu_op_2_reg_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
) -> Result<u32, Error> {
    generate_alu_op_3_reg_reg_reg(mnemonic, modifiers, register1, register1, register2)
}

fn generate_alu_op_2_reg_num(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    number: &Spanned<&Number>,
) -> Result<u32, Error> {
    generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register, register, number)
}

fn generate_alu_op_2_num(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    number: &Spanned<&Number>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            generate_alu_op_2_num_reg(mnemonic, modifiers, number, &register)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_alu_op_2_num_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    number: &Spanned<&Number>,
    register: &Spanned<&Register>,
) -> Result<u32, Error> {
    generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, register)
}

fn generate_alu_op_3(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].span_to(operands[0].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            generate_alu_op_3_reg(mnemonic, modifiers, &register, operands, symbol_table)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_alu_op_3_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[1].span_to(operands[1].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Number(number) => {
            let number = operand.span_to(number).unwrap();
            generate_alu_op_3_reg_num(
                mnemonic,
                modifiers,
                register,
                &number,
                operands,
                symbol_table,
            )
        }
        ExpressionResult::Register(register2) => {
            let register2 = operand.span_to(register2).unwrap();
            generate_alu_op_3_reg_reg(
                mnemonic,
                modifiers,
                register,
                &register2,
                operands,
                symbol_table,
            )
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register"],
            &operand.as_ref(),
        )),
    }
}

fn generate_alu_op_3_reg_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[2].span_to(operands[2].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Number(number) => {
            let number = operand.span_to(number).unwrap();
            generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register1, register2, &number)
        }
        ExpressionResult::Register(register3) => {
            let register3 = operand.span_to(register3).unwrap();
            generate_alu_op_3_reg_reg_reg(mnemonic, modifiers, register1, register2, &register3)
        }
        _ => Err(Error::incorrect_type(
            vec!["number", "register"],
            &operand.as_ref(),
        )),
    }
}

fn generate_alu_op_3_reg_reg_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
    register3: &Spanned<&Register>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= mnemonic.generate();
    opcode |= generator::generate_modifiers_alu(modifiers)?;
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= register3.generate() << 4;
    Ok(opcode)
}

fn generate_alu_op_3_reg_reg_num(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    register2: &Spanned<&Register>,
    number: &Spanned<&Number>,
) -> Result<u32, Error> {
    generator::assert_range(&number.as_u32().copied(), 0..(1 << 8))?;

    let mut opcode = 0;
    opcode |= mnemonic.generate();
    opcode |= generator::generate_modifiers_alu(modifiers)?;
    opcode |= AluOpFlags::Immediate.generate();
    opcode |= register1.generate() << 12;
    opcode |= register2.generate() << 8;
    opcode |= number.as_u32().val & 0xff;
    Ok(opcode)
}

fn generate_alu_op_3_reg_num(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
    number: &Spanned<&Number>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[2].span_to(operands[2].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register2) => {
            let register2 = operand.span_to(register2).unwrap();
            generate_alu_op_3_reg_num_reg(mnemonic, modifiers, register, number, &register2)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_alu_op_3_reg_num_reg(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register1: &Spanned<&Register>,
    number: &Spanned<&Number>,
    register2: &Spanned<&Register>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_alu_op_3_reg_reg_num(mnemonic, modifiers, register1, register2, number)?;
    opcode |= AluOpFlags::Reverse.generate();
    Ok(opcode)
}
