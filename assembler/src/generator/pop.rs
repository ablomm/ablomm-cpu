use crate::error::*;
use crate::generator::Generatable;
use crate::parser::*;

pub fn generate_pop(operation: &Spanned<Operation>) -> Result<u32, Error> {
    let mut opcode: u32 = 0;
    if operation.parameters.len() != 1 {
        return Err(Error::new(
            "Expected 1 parameter",
            operation.parameters.span,
        ));
    }
    opcode |= operation.full_mnemonic.modifiers.generate() & (0b1111 << 28);

    match &operation.parameters[0].val {
        Parameter::Register(register) => generate_pop_reg(&register, opcode),
        _ => {
            return Err(Error::new(
                "Expected a register",
                operation.parameters[0].span,
            ))
        }
    }
}

fn generate_pop_reg(register: &Register, mut opcode: u32) -> Result<u32, Error> {
    opcode |= Mnemonic::POP.generate();
    opcode |= register.generate() << 20;
    return Ok(opcode);
}
