use crate::generator::*;

pub fn generate_pop(operation: &Spanned<Operation>) -> Result<u32, Error> {
    if operation.parameters.len() != 1 {
        return Err(Error::new(
            "Expected 1 parameter",
            operation.parameters.span,
        ));
    }

    match &operation.parameters[0].val {
        Parameter::Register(register) => {
            generate_pop_reg(&operation.full_mnemonic.modifiers, &register)
        }
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_pop_reg(
    modifiers: &Spanned<Vec<Spanned<Modifier>>>,
    register: &Register,
) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    opcode |= generate_modifiers_non_alu(modifiers)?;
    opcode |= Mnemonic::POP.generate();
    opcode |= register.generate() << 20;
    return Ok(opcode);
}
