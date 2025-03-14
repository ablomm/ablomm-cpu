use super::*;

// a lot of the generation here is delegated to alu_op because, internally, a unary operation uses
// the least significant bits for it's input (i.e. the second input of a binary operation)

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
            &mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.operands,
            symbol_table,
        )
    } else if operation.operands.len() == 2 {
        generate_unary_alu_op_2(
            &mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.operands,
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
    mnemonic: &Spanned<CpuMnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[0].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register) => {
            let register = &register.unwrap();
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register)
        }
        _ => Err(SpannedError::incorrect_value(
            operands[0].span,
            "type",
            vec!["register"],
            Some(operand),
        )),
    }
}

fn generate_unary_alu_op_2(
    mnemonic: &Spanned<CpuMnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    let operand = operands[0].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register) => {
            let register = &register.unwrap();
            generate_alu_op_2_reg(mnemonic, modifiers, register, operands, symbol_table)
        }
        _ => Err(SpannedError::incorrect_value(
            operands[0].span,
            "type",
            vec!["register"],
            Some(operand),
        )),
    }
}
