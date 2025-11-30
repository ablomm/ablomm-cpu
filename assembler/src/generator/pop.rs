use crate::{
    ast::{AsmMnemonic, CpuMnemonic, Modifier, Operation, Register},
    error::Error,
    expression::expression_result::ExpressionResult,
    generator::{self, Generatable},
    span::Spanned,
    symbol_table::SymbolTable,
};

pub fn generate_pop(
    operation: &Spanned<&Operation>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Pop
    ));

    if operation.operands.len() != 1 {
        return Err(Error::incorrect_num(
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
            let register = operand.span_to(register).unwrap();
            generate_pop_reg(&operation.full_mnemonic.modifiers.as_ref(), &register)
        }
        _ => Err(Error::incorrect_type(vec!["register"], &operand.as_ref())),
    }
}

fn generate_pop_reg(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
    register: &Spanned<&Register>,
) -> Result<u32, Error> {
    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(modifiers)?;
    opcode |= CpuMnemonic::Pop.generate();
    opcode |= register.generate() << 16;
    Ok(opcode)
}
