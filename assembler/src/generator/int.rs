use crate::generator::*;
use crate::Error;

pub fn generate_int(operation: &Spanned<&Operation>) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Int
    ));

    if operation.operands.len() != 0 {
        return Err(Error::incorrect_num(
            operation.operands.span,
            "operand",
            vec![0],
            operation.operands.len(),
        ));
    }

    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(&operation.full_mnemonic.modifiers)?;
    opcode |= CpuMnemonic::Int.generate();
    Ok(opcode)
}
