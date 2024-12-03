use crate::generator::*;
use crate::Error;

pub fn generate_int(operation: &Spanned<&Operation>) -> Result<u32, Error> {
    if operation.parameters.len() != 0 {
        return Err(Error::new(
            format!("Expected {} parameters", "0".fg(ATTENTION_COLOR)),
            operation.parameters.span,
        ));
    }

    let mut opcode = 0;
    opcode |= generate_modifiers_non_alu(&operation.full_mnemonic.modifiers)?;
    opcode |= Mnemonic::Int.generate();
    Ok(opcode)
}
