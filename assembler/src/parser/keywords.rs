use super::*;

pub fn mnemonic_parser() -> impl Parser<char, AsmMnemonic, Error = ParseError> {
    return choice((
        just("nop").to(AsmMnemonic::Nop),
        just("ld").to(AsmMnemonic::Ld),
        just("push").to(AsmMnemonic::Push),
        just("pop").to(AsmMnemonic::Pop),
        just("int").to(AsmMnemonic::Int),
        just("and").to(AsmMnemonic::BinaryAlu(CpuMnemonic::And)),
        just("or").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Or)),
        just("xor").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Xor)),
        just("not").to(AsmMnemonic::UnaryAlu(CpuMnemonic::Not)),
        just("add").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Add)),
        just("addc").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Addc)),
        just("sub").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Sub)),
        just("subb").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Subb)),
        just("neg").to(AsmMnemonic::UnaryAlu(CpuMnemonic::Neg)),
        just("shl").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Shl)),
        just("shr").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Shr)),
        just("ashr").to(AsmMnemonic::BinaryAlu(CpuMnemonic::Ashr)),
    ));
}

pub fn register_parser() -> impl Parser<char, Register, Error = ParseError> {
    return choice((
        just("r0").to(Register::R0),
        just("r1").to(Register::R1),
        just("r2").to(Register::R2),
        just("r3").to(Register::R3),
        just("r4").to(Register::R4),
        just("r5").to(Register::R5),
        just("r6").to(Register::R6),
        just("r7").to(Register::R7),
        just("r8").to(Register::R8),
        just("r9").to(Register::R9),
        just("r10").to(Register::R10),
        just("fp").to(Register::R10), // just an alias
        just("status").to(Register::Status),
        just("sp").to(Register::Sp),
        just("lr").to(Register::Lr),
        just("pc.link").to(Register::Pclink), // psuedo register, used to jump with link
        just("pc").to(Register::Pc),
    ));
}

pub fn alu_modifier_parser() -> impl Parser<char, AluModifier, Error = ParseError> {
    return choice((just("s").to(AluModifier::S), just("t").to(AluModifier::T)));
}

pub fn condition_parser() -> impl Parser<char, Condition, Error = ParseError> {
    return choice((
        just("eq").to(Condition::Eq),
        just("ne").to(Condition::Ne),
        just("neg").to(Condition::Neg),
        just("pos").to(Condition::Pos),
        just("vs").to(Condition::Vs),
        just("vc").to(Condition::Vc),
        just("ult").to(Condition::Ult),
        just("ugt").to(Condition::Ugt),
        just("ule").to(Condition::Ule),
        just("uge").to(Condition::Uge),
        just("slt").to(Condition::Slt),
        just("sgt").to(Condition::Sgt),
        just("sle").to(Condition::Sle),
        just("sge").to(Condition::Sge),
        // condition aliases
        just("ns").to(Condition::Neg),
        just("nc").to(Condition::Pos),
        just("zs").to(Condition::Eq),
        just("zc").to(Condition::Ne),
        just("cs").to(Condition::Uge),
        just("cc").to(Condition::Ult),
    ));
}
