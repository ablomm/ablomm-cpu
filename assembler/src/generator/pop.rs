use ariadne::Fmt;

use crate::{expression::expression_result::ExpressionResult, generator::*};

pub fn generate_pop(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Pop
    ));

    if operation.operands.len() != 1 {
        return Err(
            Error::new(operation.operands.span, "Incorrect number of operands")
                .with_label(format!("Expected {} operands", "1".fg(ATTENTION_COLOR))),
        );
    }

    let operand = operation.operands[0].as_ref().eval(symbol_table)?.result;
    match &operand {
        ExpressionResult::Register(register) => {
            let register = &register.unwrap();
            generate_pop_reg(&operation.full_mnemonic.modifiers, register)
        }
        _ => Err(
            Error::new(operation.operands[0].span, "Incorrect type").with_label(format!(
                "Expected a {}, but found {}",
                "register".fg(ATTENTION_COLOR),
                operand.fg(ATTENTION_COLOR)
            )),
        ),
    }
}

fn generate_pop_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Pop.generate();
    opcode |= register.generate() << 16;
    Ok(opcode)
}
