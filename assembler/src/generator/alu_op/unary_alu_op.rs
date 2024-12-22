use super::*;

// a lot of the generation here is delegated to alu_op because, internally, a unary operation uses
// the least significant bits for it's input (i.e. the second input of a binary operation)

pub fn generate_unary_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.operands.len() == 1 {
        generate_unary_alu_op_1(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.operands,
            symbol_table,
        )
    } else if operation.operands.len() == 2 {
        generate_unary_alu_op_2(
            &operation.full_mnemonic.mnemonic,
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
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    operands: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    let operand = operands[0].as_ref().eval(symbol_table)?;
    match &operand.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register)
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

fn generate_unary_alu_op_2(
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
