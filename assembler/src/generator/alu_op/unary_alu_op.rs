use super::*;

// a lot of the generation here is delegated to alu_op because, internally, a unary operation uses
// the least significant bits for it's input (i.e. the second input of a binary operation)

pub fn generate_unary_alu_op(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if operation.parameters.len() == 1 {
        generate_unary_alu_op_1(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        )
    } else if operation.parameters.len() == 2 {
        generate_unary_alu_op_2(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        )
    } else {
        Err(Error::new(
            format!(
                "Expected {} or {} parameters",
                "1".fg(ATTENTION_COLOR),
                "2".fg(ATTENTION_COLOR)
            ),
            operation.parameters.span,
        ))
    }
}

fn generate_unary_alu_op_1(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register)
        }
        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            parameters[0].span,
        )),
    }
}

fn generate_unary_alu_op_2(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Expression>>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    match &parameters[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => {
            generate_alu_op_2_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            parameters[0].span,
        )),
    }
}
