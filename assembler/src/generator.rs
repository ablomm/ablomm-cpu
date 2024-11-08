use crate::parser::*;
use std::collections::HashMap;

pub trait Generatable {
    fn generate(&self) -> u32;
}

// generatable with a symbol table
pub trait GeneratableSym {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> u32;
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
        return (*self as u32) << 20;
    }
}

impl Generatable for AluModifier {
    fn generate(&self) -> u32 {
        match self {
            AluModifier::S => AluOpFlags::Load.generate() | AluOpFlags::SetStatus.generate(),
            AluModifier::T => AluOpFlags::SetStatus.generate(),
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
        return (*self as u32) << 24;
    }
}

pub fn generate(ast: Vec<Statement>) -> String {
    let mut machine_code: String = "".to_owned();
    let (symbol_table, operations) = pre_process(ast);

    for operation in operations {
        machine_code.push_str(&format!("{:x}\n", operation.generate(&symbol_table)).to_owned());
    }
    return machine_code;
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
        }
    }

    return (symbol_table, operations);
}

impl GeneratableSym for Operation {
    fn generate(&self, symbol_table: &HashMap<String, u32>) -> u32 {
        let mut opcode: u32 = 0;

        match self.full_mnemonic.mnemonic {
            Mnemonic::LD => {
                assert!(self.parameters.len() == 2, "Expected LD with 2 parameters");
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                if let Parameter::Register(register) = self.parameters[0] {
                    if let Parameter::Indirect(parameter) = &self.parameters[1] {
                        if let Parameter::Number(number) = **parameter {
                            //LD
                            opcode |= Mnemonic::LD.generate();
                            opcode |= register.generate() << 16;
                            opcode |= number & 0xffff;
                            return opcode;
                        } else if let Parameter::Register(register2) = **parameter {
                            //LDR
                            opcode |= Mnemonic::LDR.generate();
                            opcode |= register.generate() << 16;
                            opcode |= register2.generate() << 12;
                            return opcode;
                        } else if let Parameter::Label(label) = &**parameter {
                            //LD
                            opcode |= Mnemonic::LD.generate();
                            opcode |= register.generate() << 16;
                            if let Some(label_line) = symbol_table.get(&*label) {
                                opcode |= label_line & 0xffff;
                                return opcode;
                            }
                            panic!("Could not find label in LD");
                        }
                        panic!("LD only supports indirect constants, registers, and labels");
                    } else if let Parameter::Register(register2) = self.parameters[1] {
                        //MOV
                        opcode |= Mnemonic::PASSA.generate();
                        opcode |= register.generate() << 16;
                        opcode |= register2.generate() << 8;
                        return opcode;
                    } else if let Parameter::Number(number) = self.parameters[1] {
                        //LDI
                        opcode |= Mnemonic::LDI.generate();
                        opcode |= register.generate() << 16;
                        opcode |= number & 0xffff;
                        return opcode;
                    } else if let Parameter::Label(label) = &self.parameters[1] {
                        //LDI
                        opcode |= Mnemonic::LDI.generate();
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
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                if let Parameter::Register(register) = self.parameters[0] {
                    if let Parameter::Indirect(parameter) = &self.parameters[1] {
                        if let Parameter::Number(number) = **parameter {
                            //ST
                            opcode |= Mnemonic::ST.generate();
                            opcode |= 0x13 << 20;
                            opcode |= register.generate() << 16;
                            opcode |= number & 0xffff;
                            return opcode;
                        } else if let Parameter::Register(register2) = **parameter {
                            //STR
                            opcode |= Mnemonic::STR.generate();
                            opcode |= register.generate() << 16;
                            opcode |= register2.generate() << 12;
                            return opcode;
                        } else if let Parameter::Label(label) = &**parameter {
                            //ST
                            opcode |= Mnemonic::ST.generate();
                            opcode |= register.generate() << 16;
                            if let Some(label_line) = symbol_table.get(&*label) {
                                opcode |= label_line & 0xffff;
                                return opcode;
                            }
                            panic!("Could not find label in ST");
                        }
                        panic!("ST only supports indirect constants, registers, and labels");
                    } else if let Parameter::Register(register2) = self.parameters[1] {
                        //MOVR
                        opcode |= Mnemonic::PASSA.generate();
                        opcode |= register2.generate() << 16;
                        opcode |= register.generate() << 8;
                        return opcode;
                    } else if let Parameter::Number(number) = self.parameters[1] {
                        //STI
                        opcode |= Mnemonic::STI.generate();
                        opcode |= register.generate() << 16;
                        opcode |= number & 0xffff;
                        return opcode;
                    } else if let Parameter::Label(label) = &self.parameters[1] {
                        //STI
                        opcode |= Mnemonic::STI.generate();
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
            Mnemonic::PUSH => {
                assert!(self.parameters.len() == 1, "Expected PUSH with 1 parameter");
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                if let Parameter::Register(register) = self.parameters[0] {
                    opcode |= Mnemonic::PUSH.generate();
                    opcode |= register.generate() << 16;
                    return opcode;
                } else {
                    panic!("Expected PUSH parameter to be a register");
                }
            }
            Mnemonic::POP => {
                assert!(self.parameters.len() == 1, "Expected POP with 1 parameter");
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                if let Parameter::Register(register) = self.parameters[0] {
                    opcode |= Mnemonic::POP.generate();
                    opcode |= register.generate() << 20;
                    return opcode;
                } else {
                    panic!("Expected POP parameter to be a register");
                }
            }
            Mnemonic::INT => {
                assert!(self.parameters.len() == 0, "Expected INT with 0 parameter");
                opcode |= self.full_mnemonic.modifiers.generate() & (0b1111 << 28);

                opcode |= Mnemonic::INT.generate();
                return opcode;
            }
            // alu ops
            _ => {
                assert!(
                    self.parameters.len() == 2 || self.parameters.len() == 3,
                    "Expected ALU op with 2 or 3 parameter"
                );

                opcode |= self.full_mnemonic.modifiers.generate();
                opcode |= self.full_mnemonic.mnemonic.generate();

                if let Parameter::Register(register) = self.parameters[0] {
                    if self.parameters.len() == 2 {
                        if let Parameter::Register(register2) = self.parameters[1] {
                            opcode |= register.generate() << 12;
                            opcode |= register.generate() << 8;
                            opcode |= register2.generate() << 4;
                            return opcode;
                        } else if let Parameter::Number(number) = self.parameters[1] {
                            opcode |= register.generate() << 12;
                            opcode |= register.generate() << 8;
                            opcode |= number & 0xff;
                            return opcode;
                        } else if let Parameter::Label(label) = &self.parameters[1] {
                            opcode |= register.generate() << 12;
                            opcode |= register.generate() << 8;
                            if let Some(label_line) = symbol_table.get(&*label) {
                                opcode |= label_line & 0xff;
                                return opcode;
                            } else {
                                panic!("Could not find label in LD");
                            }
                        } else {
                            panic!("Expected ALU op to have second parameters of either register, number, or label");
                        }
                    } else if self.parameters.len() == 3 {
                        if let Parameter::Register(register2) = self.parameters[1] {
                            if let Parameter::Register(register3) = self.parameters[2] {
                                opcode |= register.generate() << 12;
                                opcode |= register2.generate() << 8;
                                opcode |= register3.generate() << 4;
                                return opcode;
                            } else if let Parameter::Number(number) = self.parameters[2] {
                                opcode |= register.generate() << 12;
                                opcode |= register2.generate() << 8;
                                opcode |= AluOpFlags::Immediate.generate();
                                opcode |= number & 0xff;
                                return opcode;
                            } else if let Parameter::Label(label) = &self.parameters[2] {
                                opcode |= register.generate() << 12;
                                opcode |= register2.generate() << 8;
                                opcode |= AluOpFlags::Immediate.generate();
                                if let Some(label_line) = symbol_table.get(&*label) {
                                    opcode |= label_line & 0xff;
                                    return opcode;
                                } else {
                                    panic!("Could not find label in LD");
                                }
                            } else {
                                panic!("Expected ALU op to have third parameters of either register, number, or label");
                            }
                        } else if let Parameter::Number(number) = self.parameters[1] {
                            opcode |= AluOpFlags::Reverse.generate();
                            opcode |= AluOpFlags::Immediate.generate();

                            if let Parameter::Register(register2) = self.parameters[2] {
                                opcode |= register.generate() << 12;
                                opcode |= register2.generate() << 8;
                                opcode |= number & 0xff;
                                return opcode;
                            } else {
                                panic!("Expected ALU op to have third parameter of register");
                            }
                        } else if let Parameter::Label(label) = &self.parameters[1] {
                            if let Parameter::Register(register2) = self.parameters[2] {
                                if let Some(label_line) = symbol_table.get(&*label) {
                                    opcode |= register.generate() << 12;
                                    opcode |= register2.generate() << 8;
                                    opcode |= label_line & 0xff;
                                    return opcode;
                                } else {
                                    panic!("Could not find label in LD");
                                }
                            } else {
                                panic!("Expected ALU op to have third parameter of register");
                            }
                        } else {
                            panic!("Expected ALU op to have third parameters of either register or immediate or label");
                        }
                    } else {
                        // will never reach here
                        panic!("Expected first ALU op with 2 or 3 parameters");
                    }
                } else {
                    panic!("Expected first ALU op parameter to be a register");
                }
            }
        }
    }
}
