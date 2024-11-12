use crate::generator::*;
use crate::Error;

pub fn generate_int(operation: &Spanned<Operation>) -> Result<u32, Error> {
    if operation.parameters.len() != 0 {
        return Err(Error::new(
            "Expected 0 parameters",
            operation.parameters.span,
        ));
    }

    let (conditions, alu_modifiers) = seperate_modifiers(&operation.full_mnemonic.modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            "Multiple conditions is not supported",
            operation.full_mnemonic.modifiers.span,
        ));
    }
    if alu_modifiers.len() > 0 {
        return Err(Error::new(
            "Modifier is not supported on this instruction",
            alu_modifiers[0].span,
        ));
    }

    let mut opcode: u32 = 0;
    opcode |= conditions.generate();

    opcode |= Mnemonic::INT.generate();
    return Ok(opcode);
}
