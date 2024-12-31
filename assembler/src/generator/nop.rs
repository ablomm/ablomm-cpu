use crate::generator::*;
use crate::Error;

pub fn generate_nop(operation: &Spanned<&Operation>) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Nop
    ));

    if operation.operands.len() != 0 {
        return Err(
            Error::new(operation.operands.span, "Incorrect number of operands")
                .with_label(format!("Expected {} operands", "0".fg(ATTENTION_COLOR))),
        );
    }

    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(&operation.full_mnemonic.modifiers)?;
    opcode |= CpuMnemonic::Nop.generate();
    Ok(opcode)
}
