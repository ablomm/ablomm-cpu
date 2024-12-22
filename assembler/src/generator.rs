use crate::ast::*;
use crate::error::*;
use crate::expression::expression_result::ExpressionResult;
use crate::generator::alu_op::unary_alu_op::*;
use crate::generator::alu_op::*;
use crate::generator::int::*;
use crate::generator::ld::*;
use crate::generator::pop::*;
use crate::generator::push::*;
use crate::generator::st::*;
use crate::symbol_table::SymbolTable;
use ariadne::Fmt;
use nop::*;
use std::fmt::Display;
use std::ops::Range;

mod alu_op;
mod int;
mod ld;
mod nop;
mod pop;
mod push;
mod st;

// cannot include span because blocks may span multiple different files
pub fn compile_ast(ast: &Ast) -> Result<String, Error> {
    let mut machine_code: String = "".to_owned();

    for opcode in ast.generate()? {
        machine_code.push_str(&format!("{:0>8x}\n", opcode));
    }

    Ok(machine_code)
}

// no span because ast can span many different files
impl Ast {
    fn generate(&self) -> Result<Vec<u32>, Error> {
        let mut opcodes = Vec::new();

        for file in &self.files {
            opcodes.append(&mut file.span_to(&file.block).generate()?);
        }

        Ok(opcodes)
    }
}

impl Spanned<&Block> {
    fn generate(&self) -> Result<Vec<u32>, Error> {
        let mut opcodes = Vec::new();
        for statement in &self.statements {
            opcodes.append(&mut statement.as_ref().generate(&self.symbol_table.borrow())?);
        }

        Ok(opcodes)
    }
}

impl Spanned<&Statement> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, Error> {
        match &self.val {
            Statement::Operation(operation) => self.span_to(operation).generate(symbol_table),
            Statement::Block(block) => self.span_to(block).generate(),
            Statement::Literal(literal) => self.span_to(literal).generate(symbol_table),
            _ => Ok(vec![]),
        }
    }
}

impl Spanned<&Operation> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, Error> {
        match self.full_mnemonic.mnemonic.val {
            Mnemonic::Nop => generate_nop(self),
            Mnemonic::Ld => generate_ld(self, symbol_table),
            Mnemonic::St => generate_st(self, symbol_table),
            Mnemonic::Push => generate_push(self, symbol_table),
            Mnemonic::Pop => generate_pop(self, symbol_table),
            Mnemonic::Int => generate_int(self),
            // alu ops
            Mnemonic::Not | Mnemonic::Neg => generate_unary_alu_op(self, symbol_table),
            _ => generate_alu_op(self, symbol_table),
        }
        .map(|opcode| vec![opcode])
    }
}

impl Spanned<&Expression> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, Error> {
        let result = self.eval(symbol_table)?;
        match &result.val {
            ExpressionResult::Number(number) => Ok(vec![**number]),
            ExpressionResult::String(string) => {
                let mut opcodes = Vec::new();
                // each character is 8 bytes, so we need to pack 4 in each word (as memory is word
                // addressible, not byte addressible)
                for chunk in string.as_bytes().chunks(4) {
                    let mut opcode: u32 = 0;
                    // big endian, although not technically since it all exists in the same memory
                    // address
                    for (i, c) in chunk.iter().enumerate() {
                        opcode |= (*c as u32) << (i * 8);
                    }
                    opcodes.push(opcode);
                }
                Ok(opcodes)
            }
            _ => Err(Error::new(
                format!(
                    "Expected a {} or {}, but found {}",
                    "number".fg(ATTENTION_COLOR),
                    "string".fg(ATTENTION_COLOR),
                    result.val.fg(ATTENTION_COLOR)
                ),
                self.span,
            )),
        }
    }
}

fn seperate_modifiers(
    modifiers: &Vec<Spanned<Modifier>>,
) -> (Vec<Spanned<Condition>>, Vec<Spanned<AluModifier>>) {
    let mut conditions = Vec::new();
    let mut alu_modifiers = Vec::new();

    for modifier in modifiers {
        match modifier.val {
            Modifier::Condition(condition) => conditions.push(modifier.span_to(condition)),
            Modifier::AluModifier(alu_modifier) => {
                alu_modifiers.push(modifier.span_to(alu_modifier))
            }
        }
    }

    (conditions, alu_modifiers)
}

fn generate_modifiers_non_alu(modifiers: &Spanned<Vec<Spanned<Modifier>>>) -> Result<u32, Error> {
    let (conditions, alu_modifiers) = seperate_modifiers(&modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            format!(
                "Multiple {} is not supported",
                "conditions".fg(ATTENTION_COLOR)
            ),
            conditions[1].span,
        )
        .with_note("Try removing this condition"));
    }
    if !alu_modifiers.is_empty() {
        return Err(Error::new(
            format!(
                "{} is not supported on this instruction",
                "Alu modifiers".fg(ATTENTION_COLOR)
            ),
            alu_modifiers[0].span,
        )
        .with_note("Try removing this modifier"));
    }

    Ok(conditions.generate())
}

fn generate_modifiers_alu(modifiers: &Spanned<Vec<Spanned<Modifier>>>) -> Result<u32, Error> {
    let (conditions, alu_modifiers) = seperate_modifiers(&modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            format!(
                "Multiple {} is not supported",
                "conditions".fg(ATTENTION_COLOR)
            ),
            conditions[1].span,
        )
        .with_note("Try removing this condition"));
    }
    if alu_modifiers.len() > 1 {
        return Err(Error::new(
            format!(
                "Multiple {} is not supported",
                "alu modifiers".fg(ATTENTION_COLOR)
            ),
            alu_modifiers[1].span,
        )
        .with_note("Try removing this modifier"));
    }

    Ok(conditions.generate() | alu_modifiers.generate())
}

// asserts the number is in the given range, range is inclusive of lower value, exclusive of higher
fn assert_range<T: Display + PartialOrd>(
    number: &Spanned<T>,
    range: Range<T>,
) -> Result<(), Error> {
    if !range.contains(&number.val) {
        return Err(Error::new(
            format!(
                "Only argument in range of [{}, {}) is supported; expression evaluates to {}",
                range.start.to_string().fg(ATTENTION_COLOR),
                range.end.to_string().fg(ATTENTION_COLOR),
                number.val.to_string().fg(ATTENTION_COLOR),
            ),
            number.span,
        )
        .with_note("If you require a range larger than this, use a register instead"));
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

impl Generatable for Mnemonic {
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

impl Generatable for Vec<Spanned<Modifier>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for modifier in self {
            opcode |= modifier.generate();
        }

        opcode
    }
}

impl Generatable for Vec<Spanned<Condition>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for condition in self {
            opcode |= condition.generate();
        }

        opcode
    }
}

impl Generatable for Vec<Spanned<AluModifier>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for alu_modifier in self {
            opcode |= alu_modifier.generate();
        }

        opcode
    }
}
