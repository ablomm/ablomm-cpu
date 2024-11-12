use crate::generator::*;

pub fn generate_push(operation: &Spanned<Operation>) -> Result<u32, Error> {
    if operation.parameters.len() != 1 {
        return Err(Error::new(
            "Expected 1 parameter",
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

    match &operation.parameters[0].val {
        Parameter::Register(register) => generate_push_reg(register, opcode),
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_push_reg(register: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= Mnemonic::PUSH.generate();
    opcode |= register.generate() << 16;
    return Ok(opcode);
}
