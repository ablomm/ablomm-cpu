use super::*;

// a lot of the generation here is delegated to alu_op because, internally, a unary operation uses
// the least significant bits for it's input (i.e. the second input of a binary operation)

pub fn generate_unary_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let mnemonic = if let AsmMnemonic::UnaryAlu(mnemonic) = operation.full_mnemonic.mnemonic.val {
        operation.full_mnemonic.mnemonic.span_to(mnemonic)
    } else {
        panic!("Function must be called with AsmMnemonic::UnaryAlu");
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
        Err(Error::new(
            format!(
                "Expected {} or {} operands",
                "1".fg(ATTENTION_COLOR),
                "2".fg(ATTENTION_COLOR)
            ),
            operation.operands.span,
        ))
    }
}

fn generate_unary_alu_op_1(
    mnemonic: &Spanned<CpuMnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].as_ref().eval(symbol_table)?;
    match &operand {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register)
        }
        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.fg(ATTENTION_COLOR)
            ),
            operands[0].span,
        )),
    }
}

fn generate_unary_alu_op_2(
    mnemonic: &Spanned<CpuMnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].as_ref().eval(symbol_table)?;
    match &operand {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg(mnemonic, modifiers, register, operands, symbol_table)
        }
        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.fg(ATTENTION_COLOR)
            ),
            operands[0].span,
        )),
    }
}
