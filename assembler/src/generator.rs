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
use nop::*;
use std::rc::Rc;

mod alu_op;
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

    return Ok(machine_code);
}

// cannot include span because blocks may span multiple different files
impl Block {
    fn generate(&self) -> Result<Vec<u32>, Error> {
        let mut opcodes = Vec::new();
        for statement in &self.statements {
            opcodes.append(&mut statement.as_ref().generate(&self.symbol_table.borrow())?);
        }

        return Ok(opcodes);
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
                block
                    .symbol_table
                    .borrow_mut()
                    .insert(label.to_string(), line_number as i32);
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
                    .insert(identifier.val.clone(), expression_val);
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
                                    parent_symbol_table
                                        .borrow_mut()
                                        .insert(export.val.clone(), value);
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
    return Ok(line_number);
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
                        opcode |= (*c as u32) << i * 8;
                    }
                    opcodes.push(opcode);
                }
                return Ok(opcodes);
            }
            Literal::Expression(expression) => {
                return Ok(vec![
                    Spanned::new(expression, self.span).eval(symbol_table)? as u32,
                ])
            }
        }
    }
}

fn get_identifier(ident: &Spanned<&str>, symbol_table: &SymbolTable) -> Result<i32, Error> {
    if let Some(label_line) = symbol_table.get_recursive(ident.val) {
        return Ok(label_line);
    } else {
        return Err(Error::new("Could not find identifier", ident.span));
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

fn assert_bit_length(number: &Spanned<i32>, bit_length: usize) -> Result<(), Error> {
    if number.val & (1 << bit_length) - 1 != number.val {
        return Err(Error::new(
            format!(
                "Only {} bit number supported, expression evaluates to {}, which is {} bits",
                bit_length,
                number.val as u32,
                (number.val as u32 as f32).log2().ceil()
            ),
            number.span,
        ));
    }
    return Ok(());
}

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<i32, Error> {
        match &self.val {
            // there is a bunch of deref's here (i.e. **a) because a and b are a Box, which has
            // it's own as_ref() function, but we really need the Spanned::as_ref() function. No
            // deref's are needed if the Spanned::as_ref() method is named differently, but I
            // didn't like that
            Expression::Number(a) => return Ok(*a),
            Expression::Ident(a) => {
                return get_identifier(&Spanned::new(&a, self.span), symbol_table)
            }
            Expression::Pos(a) => return Ok((**a).as_ref().eval(symbol_table)?),
            Expression::Neg(a) => return Ok(-(**a).as_ref().eval(symbol_table)?),
            Expression::Not(a) => return Ok(!(**a).as_ref().eval(symbol_table)?),
            Expression::Mul(a, b) => {
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_mul((**b).as_ref().eval(symbol_table)?))
            }
            Expression::Div(a, b) => {
                let denominator = (**b).as_ref().eval(symbol_table)?;
                if denominator == 0 {
                    return Err(Error::new("divison by 0 is undefined", b.span));
                }
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_div((**b).as_ref().eval(symbol_table)?));
            }
            Expression::Remainder(a, b) => {
                let denominator = (**b).as_ref().eval(symbol_table)?;
                if denominator == 0 {
                    return Err(Error::new("divison by 0 is undefined", b.span));
                }
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_rem((**b).as_ref().eval(symbol_table)?));
            }
            Expression::Add(a, b) => {
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_add((**b).as_ref().eval(symbol_table)?))
            }
            Expression::Sub(a, b) => {
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_sub((**b).as_ref().eval(symbol_table)?))
            }
            Expression::Shl(a, b) => {
                let b_val = (**b).as_ref().eval(symbol_table)?;
                if b_val < 0 {
                    return Err(Error::new(
                        format!("second operand cannot be negative; evaluates to {}", b_val),
                        b.span,
                    ));
                }
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_shl(b_val as u32));
            }
            Expression::Shr(a, b) => {
                let b_val = (**b).as_ref().eval(symbol_table)?;
                if b_val < 0 {
                    return Err(Error::new(
                        format!("second operand cannot be negative; evaluates to {}", b_val),
                        b.span,
                    ));
                }
                // rust will use normal shift right on unsigned types
                return Ok(
                    (((**a).as_ref().eval(symbol_table)? as u32).wrapping_shr(b_val as u32)) as i32,
                );
            }
            Expression::Ashr(a, b) => {
                let b_val = (**b).as_ref().eval(symbol_table)?;
                if b_val < 0 {
                    return Err(Error::new(
                        format!("second operand cannot be negative; evaluates to {}", b_val),
                        b.span,
                    ));
                }
                // rust will use arithmetic shift right on signed types
                return Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_shr(b_val as u32));
            }
            Expression::And(a, b) => {
                return Ok((**a).as_ref().eval(symbol_table)? & (**b).as_ref().eval(symbol_table)?)
            }
            Expression::Or(a, b) => {
                return Ok((**a).as_ref().eval(symbol_table)? | (**b).as_ref().eval(symbol_table)?)
            }
            Expression::Xor(a, b) => {
                return Ok((**a).as_ref().eval(symbol_table)? ^ (**b).as_ref().eval(symbol_table)?)
            }
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
