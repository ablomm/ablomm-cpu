use crate::generator::*;
use crate::Error;

pub fn generate_nop(operation: &Spanned<&Operation>) -> Result<u32, Error> {
    if operation.operands.len() != 0 {
        return Err(Error::new(
            format!("Expected {} operands", "0".fg(ATTENTION_COLOR)),
            operation.operands.span,
        ));
    }

    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(&operation.full_mnemonic.modifiers)?;
    opcode |= Mnemonic::Nop.generate();
    Ok(opcode)
}
