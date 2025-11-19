use crate::ast::{
    AluModifier, AluOpFlags, AsmMnemonic, Block, Condition, CpuMnemonic, Expression, Modifier,
    Operation, Register,
};
use crate::ast::{Ast, Statement};
use crate::error::{ATTENTION_COLOR, SpannedError};
use crate::expression::expression_result::ExpressionResult;
use crate::span::Spanned;
use crate::symbol_table::SymbolTable;
use ariadne::Fmt;
use std::fmt::Display;
use std::ops::Range;

mod alu_op;
mod int;
mod ld;
mod nop;
mod pop;
mod push;

// no span because ast can span many different files
impl Ast {
    pub fn assemble(&self) -> Result<String, Vec<SpannedError>> {
        let mut machine_code: String = "".to_owned();

        for opcode in self.generate()? {
            machine_code.push_str(&format!("{:0>8x}\n", opcode));
        }

        Ok(machine_code)
    }

    fn generate(&self) -> Result<Vec<u32>, Vec<SpannedError>> {
        let mut opcodes = Vec::new();

        for file in &self.files {
            opcodes.append(&mut file.span_to(&file.block).generate()?);
        }

        Ok(opcodes)
    }
}

impl Spanned<&Block> {
    fn generate(&self) -> Result<Vec<u32>, Vec<SpannedError>> {
        let mut opcodes = Vec::new();
        let mut errors = Vec::new();
        for statement in &self.statements {
            match statement.as_ref().generate(&self.symbol_table.borrow()) {
                Ok(mut opcode) => opcodes.append(&mut opcode),
                Err(mut error) => errors.append(&mut error),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(opcodes)
    }
}

impl Spanned<&Statement> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, Vec<SpannedError>> {
        match &self.val {
            Statement::Operation(operation) => self
                .span_to(operation)
                .generate(symbol_table)
                .map_err(|error| vec![error]),
            Statement::Block(block) => self.span_to(block).generate(),
            Statement::GenLiteral(literal) => self
                .span_to(literal)
                .generate(symbol_table)
                .map_err(|error| vec![error]),
            _ => Ok(vec![]),
        }
    }
}

impl Spanned<&Operation> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, SpannedError> {
        match self.full_mnemonic.mnemonic.val {
            AsmMnemonic::Nop => nop::generate_nop(self),
            AsmMnemonic::Ld => ld::generate_ld(self, symbol_table),
            AsmMnemonic::Push => push::generate_push(self, symbol_table),
            AsmMnemonic::Pop => pop::generate_pop(self, symbol_table),
            AsmMnemonic::Int => int::generate_int(self),
            AsmMnemonic::UnaryAlu(_) => alu_op::generate_unary_alu_op(self, symbol_table),
            AsmMnemonic::BinaryAlu(_) => alu_op::generate_alu_op(self, symbol_table),
        }
        .map(|opcode| vec![opcode])
    }
}

impl Spanned<&Expression> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, SpannedError> {
        let result = self.eval(symbol_table)?.result;

        match result {
            ExpressionResult::Number(number) => {
                // should never panic because generate occurs after symbol table is filled
                let number = number.expect("Number value is unknown");
                Ok(vec![*number])
            }
            ExpressionResult::String(string) => {
                // should never panic because generate occurs after symbol table is filled
                let string = string.expect("String value is unknown");
                let mut opcodes = Vec::new();
                // each character is 8 bytes, so we need to pack 4 in each word (as memory is word
                // addressable, not byte addressable)
                for chunk in string.as_bytes().chunks(4) {
                    let mut opcode: u32 = 0;
                    for (i, c) in chunk.iter().enumerate() {
                        opcode |= (*c as u32) << ((3 - i) * 8);
                    }
                    opcodes.push(opcode);
                }
                Ok(opcodes)
            }
            _ => Err(SpannedError::incorrect_value(
                self.span,
                "type",
                vec!["number", "string"],
                Some(result),
            )),
        }
    }
}

fn seperate_modifiers(
    modifiers: &Vec<Spanned<Modifier>>,
) -> (Vec<Spanned<&Condition>>, Vec<Spanned<&AluModifier>>) {
    let mut conditions = Vec::new();
    let mut alu_modifiers = Vec::new();

    for modifier in modifiers {
        match &modifier.val {
            Modifier::Condition(condition) => conditions.push(modifier.span_to(condition)),
            Modifier::AluModifier(alu_modifier) => {
                alu_modifiers.push(modifier.span_to(alu_modifier))
            }
        }
    }

    (conditions, alu_modifiers)
}

fn generate_modifiers_non_alu(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
) -> Result<u32, SpannedError> {
    let (conditions, alu_modifiers) = seperate_modifiers(modifiers.val);

    if conditions.len() > 1 {
        return Err(SpannedError::new(conditions[1].span, "Incorrect modifiers")
            .with_label(format!(
                "Multiple {} is not supported",
                "conditions".fg(ATTENTION_COLOR)
            ))
            .with_help("Try removing this condition"));
    }
    if !alu_modifiers.is_empty() {
        return Err(
            SpannedError::new(alu_modifiers[0].span, "Incorrect modifiers")
                .with_label(format!(
                    "{} is not supported on this instruction",
                    "Alu modifiers".fg(ATTENTION_COLOR)
                ))
                .with_help("Try removing this modifier"),
        );
    }

    Ok(conditions.generate())
}

fn generate_modifiers_alu(
    modifiers: &Spanned<&Vec<Spanned<Modifier>>>,
) -> Result<u32, SpannedError> {
    let (conditions, alu_modifiers) = seperate_modifiers(modifiers.val);

    if conditions.len() > 1 {
        return Err(SpannedError::new(conditions[1].span, "Incorrect modifiers")
            .with_label(format!(
                "Multiple {} is not supported",
                "conditions".fg(ATTENTION_COLOR)
            ))
            .with_help("Try removing this condition"));
    }
    if alu_modifiers.len() > 1 {
        return Err(
            SpannedError::new(alu_modifiers[1].span, "Incorrect modifiers")
                .with_label(format!(
                    "Multiple {} is not supported",
                    "alu modifiers".fg(ATTENTION_COLOR)
                ))
                .with_help("Try removing this modifier"),
        );
    }

    Ok(conditions.generate() | alu_modifiers.generate())
}

// asserts the number is in the given range, range is inclusive of lower value, exclusive of higher
fn assert_range<T: Display + PartialOrd>(
    number: &Spanned<T>,
    range: Range<T>,
) -> Result<(), SpannedError> {
    if !range.contains(&number.val) {
        return Err(SpannedError::new(number.span, "Immediate outside range")
            .with_label(format!(
                "Only argument in range of [{}, {}) is supported; expression evaluates to {}",
                range.start.to_string().fg(ATTENTION_COLOR),
                range.end.to_string().fg(ATTENTION_COLOR),
                number.val.to_string().fg(ATTENTION_COLOR),
            ))
            .with_help("If you require a range larger than this, use a register instead"));
    }

    Ok(())
}

pub trait Generatable {
    fn generate(&self) -> u32;
}

impl Generatable for Condition {
    fn generate(&self) -> u32 {
        (*self as u32) << 28
    }
}

impl Generatable for CpuMnemonic {
    fn generate(&self) -> u32 {
        (*self as u32) << 20
    }
}

impl Generatable for AluOpFlags {
    fn generate(&self) -> u32 {
        (*self as u32) << 16
    }
}

impl Generatable for Register {
    fn generate(&self) -> u32 {
        *self as u32
    }
}

impl Generatable for Modifier {
    fn generate(&self) -> u32 {
        match self {
            Modifier::Condition(condition) => condition.generate(),
            Modifier::AluModifier(alu_modifier) => alu_modifier.generate(),
        }
    }
}

impl Generatable for AluModifier {
    fn generate(&self) -> u32 {
        match self {
            AluModifier::S => AluOpFlags::SetStatus.generate(),
            AluModifier::T => AluOpFlags::Loadn.generate() | AluOpFlags::SetStatus.generate(),
        }
    }
}

impl Generatable for Vec<Spanned<&Modifier>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for modifier in self {
            opcode |= modifier.generate();
        }

        opcode
    }
}

impl Generatable for Vec<Spanned<&Condition>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for condition in self {
            opcode |= condition.generate();
        }

        opcode
    }
}

impl Generatable for Vec<Spanned<&AluModifier>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for alu_modifier in self {
            opcode |= alu_modifier.generate();
        }

        opcode
    }
}
