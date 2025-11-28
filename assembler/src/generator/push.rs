use crate::{
    ast::{AsmMnemonic, CpuMnemonic, Modifier, Operation, Register},
    error::SpannedError,
    expression::expression_result::ExpressionResult,
    generator::{self, Generatable},
    span::Spanned,
    symbol_table::SymbolTable,
};

pub fn generate_push(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, SpannedError> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Push
    ));

    if operation.operands.len() != 1 {
        return Err(SpannedError::incorrect_num(
            operation.operands.span,
            "operand",
            vec![1],
            operation.operands.len(),
        ));
    }

    let operand =
        operation.operands[0].span_to(operation.operands[0].as_ref().eval(symbol_table)?.result);
    match &operand.val {
        ExpressionResult::Register(register) => {
            let register = &register.expect("Expression resulted in None while generating");
            generate_push_reg(
                &operation.full_mnemonic.modifiers.as_ref(),
                &operand.span_to(register),
            )
        }
        _ => Err(SpannedError::incorrect_value(
            operand.span,
            "type",
            vec!["register"],
            Some(operand.val),
        )),
    }
}

fn generate_push_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
) -> Result<u32, SpannedError> {
    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Push.generate();
    opcode |= register.generate() << 16;
    Ok(opcode)
}
