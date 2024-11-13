use super::*;
use std::collections::HashMap;

// a lot of the generation here is delegated to alu_op because, internally, a unary operation uses
// the least significant bits for it's input (i.e. the second input of a binary operation)

pub fn generate_unary_alu_op(
    operation: &Spanned<Operation>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    if operation.parameters.len() == 1 {
        return generate_unary_alu_op_1(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
        );
    } else if operation.parameters.len() == 2 {
        return generate_unary_alu_op_2(
            &operation.full_mnemonic.mnemonic,
            &operation.full_mnemonic.modifiers,
            &operation.parameters,
            symbol_table,
        );
    } else {
        return Err(Error::new(
            "Expected 1 or 2 parameters",
            operation.parameters.span,
        ));
    }
}

fn generate_unary_alu_op_1(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
) -> Result<u32, Error> {
    match &parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_2_reg_reg(mnemonic, modifiers, register, register)
        }
        _ => return Err(Error::new("Expected a register", parameters[0].span)),
    }
}

fn generate_unary_alu_op_2(
    mnemonic: &Spanned<Mnemonic>,
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    parameters: &Spanned<Vec<Spanned<Parameter>>>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    match &parameters[0].val {
        Parameter::Register(register) => {
            return generate_alu_op_2_reg(mnemonic, modifiers, register, parameters, symbol_table)
        }
        _ => return Err(Error::new("Expected a register", parameters[0].span)),
    }
}
