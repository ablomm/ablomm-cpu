use crate::error::*;
use crate::generator::alu_op::*;
use crate::generator::int::*;
use crate::generator::ld::*;
use crate::generator::pop::*;
use crate::generator::push::*;
use crate::generator::st::*;
use crate::parser::*;
use std::collections::HashMap;

mod alu_op;
mod int;
mod ld;
mod pop;
mod push;
mod st;

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

impl Generatable for Vec<Modifier> {
    fn generate(&self) -> u32 {
        let mut opcode = 0;
        for modifier in self {
            opcode |= modifier.generate();
        }
        return opcode;
    }
}

impl Generatable for Mnemonic {
    fn generate(&self) -> u32 {
        return (*self as u32) << 20;
    }
}

pub fn generate(ast: Vec<Statement>) -> Result<String, Error> {
    let mut machine_code: String = "".to_owned();
    let (symbol_table, operations) = pre_process(ast);

    for operation in operations {
        match operation.generate(&symbol_table) {
            Ok(opcode) => machine_code.push_str(&format!("{:x}\n", opcode).to_string()),
            Err(error) => return Err(error),
        }
    }
    return Ok(machine_code);
}

// symbol table just has the label and the line associated with that label
fn pre_process(ast: Vec<Statement>) -> (HashMap<String, u32>, Vec<Operation>) {
    let mut symbol_table = HashMap::new();
    let mut line_number: u32 = 0;
    let mut operations: Vec<Operation> = Vec::new();

    for statement in ast {
        match statement {
            Statement::Label(label) => {
                symbol_table.insert(label, line_number as u32);
            }
            Statement::Operation(operation) => {
                operations.push(operation);
                line_number += 1;
            }
            _ => (),
        }
    }

    return (symbol_table, operations);
}

impl Operation {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> Result<u32, Error> {
        match self.full_mnemonic.mnemonic {
            Mnemonic::LD => generate_ld(self, symbol_table),
            Mnemonic::ST => generate_st(self, symbol_table),
            Mnemonic::PUSH => generate_push(self),
            Mnemonic::POP => generate_pop(self),
            Mnemonic::INT => generate_int(self),
            // alu ops
            _ => generate_alu_op(self, symbol_table),
        }
    }
}
