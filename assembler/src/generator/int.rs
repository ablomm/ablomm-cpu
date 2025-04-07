use crate::generator::*;
use crate::SpannedError;

pub fn generate_int(operation: &Spanned<&Operation>) -> Result<u32, SpannedError> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Int
    ));

    if operation.operands.len() != 0 {
        return Err(SpannedError::incorrect_num(
            operation.operands.span,
            "operand",
            vec![0],
            operation.operands.len(),
        ));
    }

    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(&operation.full_mnemonic.modifiers.as_ref())?;
    opcode |= CpuMnemonic::Int.generate();
    Ok(opcode)
}
