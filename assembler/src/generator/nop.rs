use crate::SpannedError;
use crate::generator::*;

pub fn generate_nop(operation: &Spanned<&Operation>) -> Result<u32, SpannedError> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Nop
    ));

    #[allow(clippy::len_zero)]
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
    opcode |= CpuMnemonic::Nop.generate();
    Ok(opcode)
}
