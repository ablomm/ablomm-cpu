use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*, symbol_table};

pub fn generate_pop(operation: &Spanned<&Operation>, symbol_table: &SymbolTable) -> Result<u32, Error> {
    if operation.parameters.len() != 1 {
        return Err(Error::new(
            format!("Expected {} parameter", "1".fg(ATTENTION_COLOR)),
            operation.parameters.span,
        ));
    }

    match &operation.parameters[0].as_ref().eval(symbol_table)?.val {
        ExpressionResult::Register(register) => {
            generate_pop_reg(&operation.full_mnemonic.modifiers, register)
        }
        _ => Err(Error::new(
            format!("Expected a {}", "register".fg(ATTENTION_COLOR)),
            operation.parameters[0].span,
        )),
    }
}

fn generate_pop_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::Pop.generate();
    opcode |= register.generate() << 16;
    Ok(opcode)
}
