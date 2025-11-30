use crate::{
    Error,
    ast::{AsmMnemonic, CpuMnemonic, Operation},
    generator::{self, Generatable},
    span::Spanned,
};

pub fn generate_int(operation: &Spanned<&Operation>) -> Result<u32, Error> {
    assert!(matches!(
        operation.full_mnemonic.mnemonic.val,
        AsmMnemonic::Int
    ));

    #[allow(clippy::len_zero)]
    if operation.operands.len() != 0 {
        return Err(Error::incorrect_num(
            operation.operands.span,
            "operand",
            vec![0],
            operation.operands.len(),
        ));
    }

    let mut opcode = 0;
    opcode |= generator::generate_modifiers_non_alu(&operation.full_mnemonic.modifiers.as_ref())?;
    opcode |= CpuMnemonic::Int.generate();
    Ok(opcode)
}
