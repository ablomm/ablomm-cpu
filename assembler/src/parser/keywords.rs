use super::*;

pub fn mnemonic_parser() -> impl Parser<char, Mnemonic, Error = Error> {
    return choice((
        text::keyword("nop").to(Mnemonic::NOP),
        text::keyword("ld").to(Mnemonic::LD),
        text::keyword("st").to(Mnemonic::ST),
        text::keyword("push").to(Mnemonic::PUSH),
        text::keyword("pop").to(Mnemonic::POP),
        text::keyword("int").to(Mnemonic::INT),
        text::keyword("and").to(Mnemonic::AND),
        text::keyword("or").to(Mnemonic::OR),
        text::keyword("xor").to(Mnemonic::XOR),
        text::keyword("not").to(Mnemonic::NOT),
        text::keyword("add").to(Mnemonic::ADD),
        text::keyword("addc").to(Mnemonic::ADDC),
        text::keyword("sub").to(Mnemonic::SUB),
        text::keyword("subb").to(Mnemonic::SUBB),
        text::keyword("neg").to(Mnemonic::NEG),
        text::keyword("shl").to(Mnemonic::SHL),
        text::keyword("shr").to(Mnemonic::SHR),
        text::keyword("ashr").to(Mnemonic::ASHR),
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
        text::keyword("fp").to(Register::FP),
        text::keyword("status").to(Register::STATUS),
        text::keyword("sp").to(Register::SP),
        text::keyword("lr").to(Register::LR),
        text::keyword("pc").to(Register::PC),
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
        text::keyword("eq").to(Condition::EQ),
        text::keyword("ne").to(Condition::NE),
        text::keyword("ltu").to(Condition::LTU),
        text::keyword("gtu").to(Condition::GTU),
        text::keyword("leu").to(Condition::LEU),
        text::keyword("geu").to(Condition::GEU),
        text::keyword("lts").to(Condition::LTS),
        text::keyword("gts").to(Condition::GTS),
        text::keyword("les").to(Condition::LES),
        text::keyword("ges").to(Condition::GES),
    ));
}
