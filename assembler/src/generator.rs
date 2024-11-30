use crate::ast::*;
use crate::error::*;
use crate::generator::alu_op::unary_alu_op::*;
use crate::generator::alu_op::*;
use crate::generator::int::*;
use crate::generator::ld::*;
use crate::generator::pop::*;
use crate::generator::push::*;
use crate::generator::st::*;
use crate::symbol_table::SymbolTable;
use internment::Intern;
use nop::*;
use std::fmt::Display;
use std::ops::Range;
use std::rc::Rc;

mod alu_op;
mod expression;
mod int;
mod ld;
mod nop;
mod pop;
mod push;
mod st;

// cannot include span because blocks may span multiple different files
pub fn compile_ast(ast: &Block) -> Result<String, Error> {
    let mut machine_code: String = "".to_owned();
    pre_process(ast, 0)?;

    let opcodes = ast.generate()?;
    for opcode in opcodes {
        machine_code.push_str(&format!("{:0>8x}\n", opcode));
    }

    Ok(machine_code)
}

// cannot include span because blocks may span multiple different files
impl Block {
    fn generate(&self) -> Result<Vec<u32>, Error> {
        let mut opcodes = Vec::new();
        for statement in &self.statements {
            opcodes.append(&mut statement.as_ref().generate(&self.symbol_table.borrow())?);
        }

        Ok(opcodes)
    }
}

// cannot include span because blocks may span multiple different files
fn pre_process(block: &Block, start_address: u32) -> Result<u32, Error> {
    let mut line_number: u32 = start_address;

    for statement in &block.statements {
        match &statement.val {
            Statement::Label(label) => {
                if block.symbol_table.borrow().contains_key(label) {
                    return Err(Error::new("Identifier already defined", statement.span));
                }
                block.symbol_table.borrow_mut().insert(*label, line_number);
            }
            Statement::Assignment(identifier, expression) => {
                if block.symbol_table.borrow().contains_key(&identifier.val) {
                    return Err(Error::new("Identifier already defined", identifier.span));
                }

                // need to move the expression evaluation out of the symbol_table.insert() call
                // to satisfy the borrow checker
                let expression_val = expression.as_ref().eval(&block.symbol_table.borrow())?;

                block
                    .symbol_table
                    .borrow_mut()
                    .insert(identifier.val, expression_val);
            }
            Statement::Operation(_) => {
                line_number += 1;
            }
            Statement::Literal(literal) => {
                line_number += literal.num_lines();
            }
            Statement::Export(exports) => {
                let symbol_table = &block.symbol_table.borrow();
                match &symbol_table.parent {
                    Some(parent_symbol_table) => {
                        for export in exports {
                            match symbol_table.get(&export.val) {
                                Some(value) => {
                                    parent_symbol_table.borrow_mut().insert(export.val, value);
                                }
                                None => {
                                    return Err(Error::new(
                                        "Could not find identifier",
                                        export.span,
                                    ));
                                }
                            }
                        }
                    }
                    None => {
                        return Err(Error::new(
                            "No parent symbol table found in this scope",
                            statement.span,
                        ));
                    }
                }
            }
            Statement::Block(sub_block) => {
                sub_block.symbol_table.borrow_mut().parent = Some(Rc::clone(&block.symbol_table));
                line_number = pre_process(&Spanned::new(sub_block, statement.span), line_number)?;
            }
            _ => (),
        }
    }

    Ok(line_number)
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

    (conditions, alu_modifiers)
}

impl Literal {
    pub fn num_lines(&self) -> u32 {
        match self {
            Literal::String(string) => ((string.len() as f32) / 4.0).ceil() as u32,
            _ => 1,
        }
    }
}

impl Spanned<&Statement> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, Error> {
        match &self.val {
            Statement::Operation(operation) => {
                Spanned::new(operation, self.span).generate(symbol_table)
            }
            Statement::Block(block) => Spanned::new(block, self.span).generate(),
            Statement::Literal(literal) => Spanned::new(literal, self.span).generate(symbol_table),
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
            Mnemonic::Push => generate_push(self),
            Mnemonic::Pop => generate_pop(self),
            Mnemonic::Int => generate_int(self),
            // alu ops
            Mnemonic::Not | Mnemonic::Neg => generate_unary_alu_op(self, symbol_table),
            _ => generate_alu_op(self, symbol_table),
        }
        .map(|opcode| vec![opcode])
    }
}

impl Spanned<&Literal> {
    fn generate(&self, symbol_table: &SymbolTable) -> Result<Vec<u32>, Error> {
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
                        opcode |= (*c as u32) << (i * 8);
                    }
                    opcodes.push(opcode);
                }
                Ok(opcodes)
            }
            Literal::Expression(expression) => {
                Ok(vec![Spanned::new(expression, self.span).eval(symbol_table)?])
            }
        }
    }
}

fn get_identifier(
    ident: &Spanned<&Intern<String>>,
    symbol_table: &SymbolTable,
) -> Result<u32, Error> {
    if let Some(label_line) = symbol_table.get_recursive(ident.val) {
        Ok(label_line)
    } else {
        Err(Error::new("Could not find identifier", ident.span))
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
    if !alu_modifiers.is_empty() {
        return Err(Error::new(
            "Alu modifiers is not supported on this instruction",
            modifiers.span,
        ));
    }

    Ok(conditions.generate())
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
                "Only argument in range of [{}, {}) is supported, expression evaluates to {}",
                range.start, range.end, number.val,
            ),
            number.span,
        ));
    }

    Ok(())
}

pub trait Generatable {
    fn generate(&self) -> u32;
}

impl Generatable for Register {
    fn generate(&self) -> u32 {
        *self as u32
    }
}

impl Generatable for Condition {
    fn generate(&self) -> u32 {
        (*self as u32) << 28
    }
}

impl Generatable for AluOpFlags {
    fn generate(&self) -> u32 {
        (*self as u32) << 16
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

impl Generatable for Mnemonic {
    fn generate(&self) -> u32 {
        (*self as u32) << 20
    }
}
