use crate::generator::*;
use crate::Error;

pub fn generate_nop(operation: &Spanned<Operation>) -> Result<u32, Error> {
    if operation.parameters.len() != 0 {
        return Err(Error::new(
            "Expected 0 parameters",
            operation.parameters.span,
        ));
    }

    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(&operation.full_mnemonic.modifiers)?;
    opcode |= Mnemonic::NOP.generate();
    return Ok(opcode);
}
