use crate::error::*;
use crate::generator::alu_op::unary_alu_op::*;
use crate::generator::alu_op::*;
use crate::generator::int::*;
use crate::generator::ld::*;
use crate::generator::pop::*;
use crate::generator::push::*;
use crate::generator::st::*;
use crate::parser::*;
use crate::span::*;
use internment::Intern;
use nop::*;
use std::collections::HashMap;

mod alu_op;
mod int;
mod ld;
mod nop;
mod pop;
mod push;
mod st;

pub fn generate(ast: Vec<Spanned<Statement>>) -> Result<String, Error> {
    let mut machine_code: String = "".to_owned();
    let (symbol_table, generatables) = pre_process(ast)?;

    for generatable in generatables {
        let opcodes = generatable.generate(&symbol_table)?;
        for opcode in opcodes {
            machine_code.push_str(&format!("{:x}\n", opcode));
        }
    }

    return Ok(machine_code);
}

// symbol table just has the label and the line associated with that label
fn pre_process(
    ast: Vec<Spanned<Statement>>,
) -> Result<(HashMap<String, u32>, Vec<Box<dyn GeneratableSym>>), Error> {
    let mut symbol_table = HashMap::new();
    let mut line_number: u32 = 0;
    let mut operations: Vec<Box<dyn GeneratableSym>> = Vec::new();

    for statement in ast {
        match statement.val {
            Statement::Label(label) => {
                if symbol_table.contains_key(&label) {
                    return Err(Error::new("Label already defined", statement.span));
                }
                symbol_table.insert(label, line_number as u32);
            }
            Statement::Operation(operation) => {
                line_number += 1;
                operations.push(Box::new(Spanned::new(operation, statement.span)));
            }
            Statement::Literal(literal) => {
                line_number += literal.num_lines();
                operations.push(Box::new(Spanned::new(literal, statement.span)));
            }
            _ => (),
        }
    }

    return Ok((symbol_table, operations));
}

fn seperate_modifiers(
    modifiers: &Vec<Spanned<Modifier>>,
) -> (Vec<Spanned<Condition>>, Vec<Spanned<AluModifier>>) {
    let mut conditions = Vec::new();
    let mut alu_modifiers = Vec::new();

    for modifier in modifiers {
        match modifier.val {
            Modifier::Condition(condition) => {
                conditions.push(Spanned::new(condition, modifier.span))
            }
            Modifier::AluModifier(alu_modifier) => {
                alu_modifiers.push(Spanned::new(alu_modifier, modifier.span))
            }
        }
    }

    return (conditions, alu_modifiers);
}

impl Literal {
    pub fn num_lines(&self) -> u32 {
        match self {
            Literal::String(string) => {
                return ((string.len() as f32) / 4.0).ceil() as u32;
            }
            _ => return 1,
        }
    }
}

pub trait GeneratableSym {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> Result<Vec<u32>, Error>;
}

impl GeneratableSym for Spanned<Operation> {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> Result<Vec<u32>, Error> {
        match self.full_mnemonic.mnemonic.val {
            Mnemonic::NOP => generate_nop(self),
            Mnemonic::LD => generate_ld(self, symbol_table),
            Mnemonic::ST => generate_st(self, symbol_table),
            Mnemonic::PUSH => generate_push(self),
            Mnemonic::POP => generate_pop(self),
            Mnemonic::INT => generate_int(self),
            // alu ops
            Mnemonic::NOT | Mnemonic::NEG => generate_unary_alu_op(self, symbol_table),
            _ => generate_alu_op(self, symbol_table),
        }
        .map(|opcode| vec![opcode])
    }
}

impl GeneratableSym for Spanned<Literal> {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> Result<Vec<u32>, Error> {
        match &self.val {
            Literal::String(string) => {
                let mut opcodes = Vec::new();
                // each character is 8 bytes, so we need to pack 4 in each word (as memory is word
                // addressible, not byte addressible)
                for chunk in string.as_bytes().chunks(4) {
                    let mut opcode: u32 = 0;
                    // big endian, although not technically since it all exists in the same memory
                    // address
                    for (i, c) in chunk.iter().enumerate() {
                        opcode |= (*c as u32) << i * 8;
                    }
                    opcodes.push(opcode);
                }
                return Ok(opcodes);
            }
            Literal::Expression(expression) => return Ok(vec![expression.eval(symbol_table)?]),
        }
    }
}

pub trait Generatable {
    fn generate(&self) -> u32;
}

impl Generatable for Register {
    fn generate(&self) -> u32 {
        return *self as u32;
    }
}

impl Generatable for Condition {
    fn generate(&self) -> u32 {
        return (*self as u32) << 28;
    }
}

impl Generatable for AluOpFlags {
    fn generate(&self) -> u32 {
        return (*self as u32) << 16;
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

impl Generatable for Modifier {
    fn generate(&self) -> u32 {
        match self {
            Modifier::Condition(condition) => condition.generate(),
            Modifier::AluModifier(alu_modifier) => alu_modifier.generate(),
        }
    }
}

impl Generatable for Vec<Spanned<Modifier>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for modifier in self {
            opcode |= modifier.generate();
        }
        return opcode;
    }
}

impl Generatable for Vec<Spanned<Condition>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for condition in self {
            opcode |= condition.generate();
        }
        return opcode;
    }
}

impl Generatable for Vec<Spanned<AluModifier>> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for alu_modifier in self {
            opcode |= alu_modifier.generate();
        }
        return opcode;
    }
}

impl Generatable for Mnemonic {
    fn generate(&self) -> u32 {
        return (*self as u32) << 20;
    }
}

fn get_label_address(
    label: &Spanned<&str>,
    symbol_table: &HashMap<String, u32>,
) -> Result<u32, Error> {
    if let Some(label_line) = symbol_table.get(label.val) {
        return Ok(*label_line);
    } else {
        return Err(Error::new("Could not find label", label.span));
    }
}

fn generate_modifiers_non_alu(modifiers: &Spanned<Vec<Spanned<Modifier>>>) -> Result<u32, Error> {
    let (conditions, alu_modifiers) = seperate_modifiers(&modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            "Multiple conditions is not supported",
            conditions[1].span,
        ));
    }
    if alu_modifiers.len() > 0 {
        return Err(Error::new(
            "Alu modifiers is not supported on this instruction",
            modifiers.span,
        ));
    }

    return Ok(conditions.generate());
}

fn generate_modifiers_alu(modifiers: &Spanned<Vec<Spanned<Modifier>>>) -> Result<u32, Error> {
    let (conditions, alu_modifiers) = seperate_modifiers(&modifiers.val);

    if conditions.len() > 1 {
        return Err(Error::new(
            "Multiple conditions is not supported",
            conditions[1].span,
        ));
    }
    if alu_modifiers.len() > 1 {
        return Err(Error::new(
            "Multiple alu modifiers is not supported",
            alu_modifiers[1].span,
        ));
    }

    return Ok(conditions.generate() | alu_modifiers.generate());
}

impl Expression {
    pub fn eval(&self, symbol_table: &HashMap<String, u32>) -> Result<u32, Error> {
        match self {
            Expression::Number(a) => return Ok(*a),
            Expression::Ident(a) => {
                return get_label_address(
                    &Spanned::new(a, Span::new(Intern::new("abc".to_string()), 2..2)),
                    symbol_table,
                )
            }
            Expression::Neg(a) => return a.eval(symbol_table), // todo negatives
            Expression::Add(a, b) => return Ok(a.eval(symbol_table)? + b.eval(symbol_table)?),
            Expression::Sub(a, b) => return Ok(a.eval(symbol_table)? - b.eval(symbol_table)?),
            Expression::Mul(a, b) => return Ok(a.eval(symbol_table)? * b.eval(symbol_table)?),
            Expression::Div(a, b) => return Ok(a.eval(symbol_table)? / b.eval(symbol_table)?),
        }
    }
}
