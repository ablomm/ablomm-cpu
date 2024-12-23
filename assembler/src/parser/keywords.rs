use super::*;

pub fn mnemonic_parser() -> impl Parser<char, AsmMnemonic, Error = Error> {
    return choice((
        text::keyword("nop").to(AsmMnemonic::Nop),
        text::keyword("ld").to(AsmMnemonic::Ld),
        text::keyword("push").to(AsmMnemonic::Push),
        text::keyword("pop").to(AsmMnemonic::Pop),
        text::keyword("int").to(AsmMnemonic::Int),
        text::keyword("and").to(AsmMnemonic::BinaryAlu(CpuMnemonic::And)),
        text::keyword("or").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Or)),
        text::keyword("xor").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Xor)),
        text::keyword("not").to(AsmMnemonic::UnaryAlu(CpuMnemonic::Not)),
        text::keyword("add").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Add)),
        text::keyword("addc").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Addc)),
        text::keyword("sub").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Sub)),
        text::keyword("subb").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Subb)),
        text::keyword("neg").to(AsmMnemonic::UnaryAlu(CpuMnemonic::Neg)),
        text::keyword("shl").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Shl)),
        text::keyword("shr").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Shr)),
        text::keyword("ashr").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Ashr)),
    ));
}

pub fn register_parser() -> impl Parser<char, Register, Error = Error> {
    return choice((
        text::keyword("r0").to(Register::R0),
        text::keyword("r1").to(Register::R1),
        text::keyword("r2").to(Register::R2),
        text::keyword("r3").to(Register::R3),
        text::keyword("r4").to(Register::R4),
        text::keyword("r5").to(Register::R5),
        text::keyword("r6").to(Register::R6),
        text::keyword("r7").to(Register::R7),
        text::keyword("r8").to(Register::R8),
        text::keyword("r9").to(Register::R9),
        text::keyword("r10").to(Register::R10),
        text::keyword("r11").to(Register::R11),
        text::keyword("fp").to(Register::R11), // just an alias
        text::keyword("status").to(Register::Status),
        text::keyword("sp").to(Register::Sp),
        text::keyword("lr").to(Register::Lr),
        text::keyword("pc").to(Register::Pc),
    ));
}

pub fn alu_modifier_parser() -> impl Parser<char, AluModifier, Error = Error> {
    return choice((
        text::keyword("s").to(AluModifier::S),
        text::keyword("t").to(AluModifier::T),
    ));
}

pub fn condition_parser() -> impl Parser<char, Condition, Error = Error> {
    return choice((
        text::keyword("eq").to(Condition::Eq),
        text::keyword("ne").to(Condition::Ne),
        text::keyword("ult").to(Condition::Ult),
        text::keyword("ugt").to(Condition::Ugt),
        text::keyword("ule").to(Condition::Ule),
        text::keyword("uge").to(Condition::Uge),
        text::keyword("slt").to(Condition::Slt),
        text::keyword("sgt").to(Condition::Sgt),
        text::keyword("sle").to(Condition::Sle),
        text::keyword("sge").to(Condition::Sge),
    ));
}
