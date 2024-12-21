use super::*;

pub fn mnemonic_parser() -> impl Parser<char, Mnemonic, Error = Error> {
    return choice((
        text::keyword("nop").to(Mnemonic::Nop),
        text::keyword("ld").to(Mnemonic::Ld),
        text::keyword("st").to(Mnemonic::St),
        text::keyword("push").to(Mnemonic::Push),
        text::keyword("pop").to(Mnemonic::Pop),
        text::keyword("int").to(Mnemonic::Int),
        text::keyword("and").to(Mnemonic::And),
        text::keyword("or").to(Mnemonic::Or),
        text::keyword("xor").to(Mnemonic::Xor),
        text::keyword("not").to(Mnemonic::Not),
        text::keyword("add").to(Mnemonic::Add),
        text::keyword("addc").to(Mnemonic::Addc),
        text::keyword("sub").to(Mnemonic::Sub),
        text::keyword("subb").to(Mnemonic::Subb),
        text::keyword("neg").to(Mnemonic::Neg),
        text::keyword("shl").to(Mnemonic::Shl),
        text::keyword("shr").to(Mnemonic::Shr),
        text::keyword("ashr").to(Mnemonic::Ashr),
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
