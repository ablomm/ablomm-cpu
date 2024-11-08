use crate::generator::ld_st::*;
use crate::generator::alu_op::*;
use crate::parser::*;
use std::collections::HashMap;

mod ld_st;
mod alu_op;

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

pub fn generate(ast: Vec<Statement>) -> Result<String, &'static str> {
    let mut machine_code: String = "".to_owned();
    let (symbol_table, operations) = pre_process(ast);

    for operation in operations {
        match operation.generate(&symbol_table) {
            Ok(opcode) => machine_code.push_str(&format!("{:x}\n", opcode).to_owned()),
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
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> Result<u32, &'static str> {
        let mut opcode: u32 = 0;

        match self.full_mnemonic.mnemonic {
            Mnemonic::LD | Mnemonic::ST => generate_ld_st(self, symbol_table),
            Mnemonic::PUSH => {
                if self.parameters.len() != 1 {
                    return Err("Expected PUSH with 1 parameter");
                }
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                if let Parameter::Register(register) = self.parameters[0] {
                    opcode |= Mnemonic::PUSH.generate();
                    opcode |= register.generate() << 16;
                    return Ok(opcode);
                } else {
                    return Err("Expected PUSH parameter to be a register");
                }
            }
            Mnemonic::POP => {
                if self.parameters.len() != 1 {
                    return Err("Expected POP with 1 parameter");
                }
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                if let Parameter::Register(register) = self.parameters[0] {
                    opcode |= Mnemonic::POP.generate();
                    opcode |= register.generate() << 20;
                    return Ok(opcode);
                } else {
                    return Err("Expected POP parameter to be a register");
                }
            }
            Mnemonic::INT => {
                if self.parameters.len() != 0 {
                    return Err("Expected INT with 0 parameters");
                }
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                opcode |= Mnemonic::INT.generate();
                return Ok(opcode);
            }
            // alu ops
            _ => generate_alu_op(self, symbol_table),
        }
    }
}
