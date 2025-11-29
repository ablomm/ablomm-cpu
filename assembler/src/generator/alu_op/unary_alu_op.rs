// a lot of the generation here is delegated to alu_op because, internally, a unary operation uses
// the least significant bits for it's input (i.e. the second input of a binary operation)

use crate::{
    ast::{AsmMnemonic, CpuMnemonic, Expression, Modifier, Operation},
    error::SpannedError,
    expression::expression_result::ExpressionResult,
    generator::alu_op,
    span::Spanned,
    symbol_table::SymbolTable,
};

pub fn generate_unary_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let mnemonic = if let AsmMnemonic::UnaryAlu(mnemonic) = operation.full_mnemonic.mnemonic.val {
        operation.full_mnemonic.mnemonic.span_to(mnemonic)
    } else {
        panic!("Function was not called with AsmMnemonic::UnaryAlu");
    };

    if operation.operands.len() == 1 {
        generate_unary_alu_op_1(
            &mnemonic.as_ref(),
            &operation.full_mnemonic.modifiers.as_ref(),
            &operation.operands.as_ref(),
            symbol_table,
        )
    } else if operation.operands.len() == 2 {
        generate_unary_alu_op_2(
            &mnemonic.as_ref(),
            &operation.full_mnemonic.modifiers.as_ref(),
            &operation.operands.as_ref(),
            symbol_table,
        )
    } else {
        Err(SpannedError::incorrect_num(
            operation.operands.span,
            "operand",
            vec![1, 2],
            operation.operands.len(),
        ))
    }
}

fn generate_unary_alu_op_1(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[0].span_to(operands[0].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            alu_op::generate_alu_op_2_reg_reg(mnemonic, modifiers, &register, &register)
        }
        _ => Err(SpannedError::incorrect_type(
            vec!["register"],
            &operand.as_ref(),
        )),
    }
}

fn generate_unary_alu_op_2(
    mnemonic: &Spanned<&CpuMnemonic>,
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    operands: &Spanned<&Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[0].span_to(operands[0].as_ref().eval(symbol_table)?.result);

    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = operand.span_to(register).unwrap();
            alu_op::generate_alu_op_2_reg(mnemonic, modifiers, &register, operands, symbol_table)
        }
        _ => Err(SpannedError::incorrect_type(
            vec!["register"],
            &operand.as_ref(),
        )),
    }
}
