use crate::parser::*;
use std::collections::HashMap;

pub fn generate(ast: Vec<Statement>) -> String {
    let mut machine_code: String = "".to_owned();
    let symbol_table = generate_symbol_table(&ast);

    let operations = ast.iter().filter_map(|statement| match statement {
        Statement::Operation(operation) => return Some(operation),
        _ => None,
    });

    for operation in operations {
        machine_code.push_str(&format!("{:x}\n", operation.generate(&symbol_table)).to_owned());
    }
    return machine_code;
}

// symbol table just has the label and the line associated with that label
fn generate_symbol_table(ast: &Vec<Statement>) -> HashMap<String, u32> {
    let mut symbol_table = HashMap::new();
    let mut line_number: u32 = 0;

    for statement in ast {
        if let Statement::Label(label) = statement {
            symbol_table.insert(label.clone(), line_number as u32);
        } else { // only operations count as a line; labels don't
            line_number += 1;
        }
    }

    return symbol_table;
}


impl Operation {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> u32 {
        let mut opcode: u32 = 0;

        // set condition bits
        for modifier in &self.full_mnemonic.modifiers {
            if let Modifier::Condition(condition) = modifier {
                opcode |= condition.generate() << 28;
                break;
            }
        }

        match self.full_mnemonic.mnemonic {
            Mnemonic::LD => {
                assert!(self.parameters.len() == 2, "Expected LD with 2 parameters");

                if let Parameter::Register(register) = self.parameters[0] {
                    if let Parameter::Indirect(parameter) = &self.parameters[1] {
                        if let Parameter::Number(number) = **parameter {
                            //LD
                            opcode |= 0x10 << 20;
                            opcode |= register.generate() << 16;
                            opcode |= number & 0xffff;
                            return opcode;
                        } else if let Parameter::Register(register2) = **parameter {
                            //LDR
                            opcode |= 0x11 << 20;
                            opcode |= register.generate() << 16;
                            opcode |= register2.generate() << 12;
                            return opcode;
                        }
                        panic!("LD only supports indirect constants or registers");
                    } else if let Parameter::Register(register2) = self.parameters[1] {
                        //MOV
                        opcode |= 0x00 << 20; // just for consistency 00 = PASSA
                        opcode |= register.generate() << 16;
                        opcode |= register2.generate() << 8;
                        return opcode;
                    } else if let Parameter::Number(number) = self.parameters[1] {
                        //LDI
                        opcode |= 0x12 << 20;
                        opcode |= register.generate() << 16;
                        opcode |= number & 0xffff;
                        return opcode;
                    } else if let Parameter::Label(label) = &self.parameters[1] {
                        //LDI
                        opcode |= 0x12 << 20;
                        opcode |= register.generate() << 16;
                        if let Some(label_line) = symbol_table.get(&*label) {
                            opcode |= label_line & 0xffff;
                            return opcode;
                        }
                        panic!("Could not find label in LD");
                    }
                    panic!(
                        "Expected second LD parameter to be either indirect, register, or number"
                    );
                }

                panic!("expected first LD parameter to be a register");
            }
            Mnemonic::ST => {
                assert!(self.parameters.len() == 2, "Expected ST with 2 parameters");

                if let Parameter::Register(register) = self.parameters[0] {
                    if let Parameter::Indirect(parameter) = &self.parameters[1] {
                        if let Parameter::Number(number) = **parameter {
                            //ST
                            opcode |= 0x13 << 20;
                            opcode |= register.generate() << 16;
                            opcode |= number & 0xffff;
                            return opcode;
                        } else if let Parameter::Register(register2) = **parameter {
                            //STR
                            opcode |= 0x14 << 20;
                            opcode |= register.generate() << 16;
                            opcode |= register2.generate() << 12;
                            return opcode;
                        }
                        panic!("LD only supports indirect constants or registers");
                    } else if let Parameter::Register(register2) = self.parameters[1] {
                        //MOVR
                        opcode |= 0x00 << 20; // just for consistency 00 = PASSA
                        opcode |= register2.generate() << 16;
                        opcode |= register.generate() << 8;
                        return opcode;
                    } else if let Parameter::Number(number) = self.parameters[1] {
                        //STI
                        opcode |= 0x15 << 20;
                        opcode |= register.generate() << 16;
                        opcode |= number & 0xffff;
                        return opcode;
                    } else if let Parameter::Label(label) = &self.parameters[1] {
                        //STI
                        opcode |= 0x12 << 20;
                        opcode |= register.generate() << 16;
                        if let Some(label_line) = symbol_table.get(&*label) {
                            opcode |= label_line & 0xffff;
                            return opcode;
                        }
                        panic!("Could not find label in LD");
                    }
                    panic!(
                        "Expected second LD parameter to be either indirect, register, or number"
                    );
                }

                panic!("expected first LD parameter to be a register");
            },
            Mnemonic::PUSH => {
                assert!(self.parameters.len() == 1, "Expected PUSH with 1 parameter");
                if let Parameter::Register(register) = self.parameters[0] {
                    opcode |= Mnemonic::PUSH.generate() << 20;
                    opcode |= register.generate() << 16;
                    return opcode;
                } else {
                    panic!("Expected PUSH parameter to be a register");
                }
            },
            Mnemonic::POP => {
                assert!(self.parameters.len() == 1, "Expected POP with 1 parameter");
                if let Parameter::Register(register) = self.parameters[0] {
                    opcode |= Mnemonic::POP.generate() << 20;
                    opcode |= register.generate() << 16;
                    return opcode;
                } else {
                    panic!("Expected POP parameter to be a register");
                }
            },
            Mnemonic::INT => {
                opcode |= Mnemonic::INT.generate() << 20;
                return opcode;
            },
            _ => {
                //todo alu
                return 3;
            }
        }
    }
}
