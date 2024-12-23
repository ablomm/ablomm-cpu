use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub fn generate_push(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Push
    ));

    if operation.operands.len() != 1 {
        return Err(Error::new(
            format!("Expected {} operands", "1".fg(ATTENTION_COLOR)),
            operation.operands.span,
        ));
    }

    let operand = operation.operands[0].as_ref().eval(symbol_table)?;
    match &operand {
        ExpressionResult::Register(register) => {
            generate_push_reg(&operation.full_mnemonic.modifiers, register)
        }
        _ => Err(Error::new(
            format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.fg(ATTENTION_COLOR)
            ),
            operation.operands[0].span,
        )),
    }
}

fn generate_push_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Push.generate();
    opcode |= register.generate() << 16;
    Ok(opcode)
}
