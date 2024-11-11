use crate::generator::Generatable;
use crate::{parser::*, Error};

pub fn generate_int(operation: &Spanned<Operation>) -> Result<u32, Error> {
    let mut opcode: u32 = 0;

    if operation.parameters.len() != 0 {
        return Err(Error::new(
            "Expected 0 parameters",
            operation.parameters.span,
        ));
    }

    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);
    opcode |= Mnemonic::INT.generate();
    return Ok(opcode);
}
