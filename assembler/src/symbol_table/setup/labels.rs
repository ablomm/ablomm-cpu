use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Ast, Block, Expression, File, Operation, Statement},
    error::{RecoveredError, RecoveredResult, SpannedError},
    expression::{
        EvalReturn,
        expression_result::{ExpressionResult, Number},
    },
    span::Spanned,
    symbol_table::SymbolTable,
};

impl Ast {
    // calculates label addresses
    pub fn set_labels(&mut self) -> Result<(), Vec<SpannedError>> {
        let mut errors = Vec::new();

        let mut address_accumulator = 0;
        for file in self.files.iter_mut() {
            address_accumulator = match file.as_mut_ref().set_labels(address_accumulator) {
                Ok(address) => address,
                Err(RecoveredError(address, mut label_errors)) => {
                    errors.append(&mut label_errors);
                    address
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Spanned<&mut File> {
    fn set_labels(&mut self, start_address: u32) -> RecoveredResult<u32> {
        self.span.spanned(&mut self.block).set_labels(start_address)
    }
}

impl Spanned<&mut Block> {
    fn set_labels(&mut self, start_address: u32) -> RecoveredResult<u32> {
        let mut address = start_address;
        let mut errors = Vec::new();

        // to satisfy borrow checker
        let symbol_table = Rc::clone(&self.symbol_table);

        // filter out statements that cause errors so that an erroneous statement doesn't cascade errors
        self.statements.retain_mut(|statement| {
            match statement.as_mut_ref().set_labels(address, &symbol_table) {
                Ok(new_address) => {
                    address = new_address;
                    true
                }
                Err(RecoveredError(new_address, mut statement_errors)) => {
                    address = new_address;
                    errors.append(&mut statement_errors);
                    false
                }
            }
        });

        if errors.is_empty() {
            Ok(address)
        } else {
            Err(RecoveredError(address, errors))
        }
    }
}

impl Spanned<&mut Statement> {
    fn set_labels(
        &mut self,
        start_address: u32,
        symbol_table: &Rc<RefCell<SymbolTable>>,
    ) -> RecoveredResult<u32> {
        let mut address = start_address;
        let mut errors = Vec::new();

        match self.val {
            Statement::Label(label) => {
                let result = label
                    .identifier
                    .span_to(ExpressionResult::Number(Some(Number(address))));

                let symbol_table = symbol_table.borrow_mut();

                let entry = symbol_table
                .get(&label.identifier.as_ref())
                .unwrap_or_else(|| {
                    panic!(
                        "Label '{}' type did not exist in symbol table when inserting final value",
                        label.identifier.val
                    )
                });

                entry.symbol.borrow_mut().result = Some(result);
            }

            Statement::Block(sub_block) => match self.span.spanned(sub_block).set_labels(address) {
                Ok(_) => (),
                Err(RecoveredError(_, mut sub_errors)) => errors.append(&mut sub_errors),
            },
            _ => (),
        }

        address += match self.to_borrow().num_words(&symbol_table.borrow()) {
            Ok(length) => length,
            Err(error) => {
                errors.push(error);
                0
            }
        };

        if errors.is_empty() {
            Ok(address)
        } else {
            Err(RecoveredError(address, errors))
        }
    }
}

impl Block {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, SpannedError> {
        let mut num_words = 0;
        for statement in &self.statements {
            num_words += statement.as_ref().num_words(symbol_table)?;
        }

        Ok(num_words)
    }
}

impl Spanned<&Statement> {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, SpannedError> {
        match self.val {
            Statement::GenLiteral(literal) => self.span_to(literal).num_words(symbol_table),
            Statement::Block(block) => block.num_words(&block.symbol_table.borrow()),
            Statement::Operation(operation) => operation.num_words(),
            _ => Ok(0),
        }
    }
}

impl Spanned<&Expression> {
    pub fn num_words(&self, symbol_table: &SymbolTable) -> Result<u32, SpannedError> {
        let EvalReturn {
            result,
            waiting_map,
        } = self.eval(symbol_table)?;

        match result {
            ExpressionResult::Number(_number) => Ok(1),
            ExpressionResult::String(string) => {
                let string = string.ok_or_else(|| {
                    let mut error = SpannedError::new(self.span, "Unknown value of expression").with_label(
                        "Expression needs to be determined, but is not",
                    );
                    for span in waiting_map.values() {
                        error = error.with_label_span(*span, "This value is undetermined")
                    }

                    error.with_note(
                        "This is ultimately caused because the expression is dependent on a future address (label), but the value of the expression would effect that address (label)",
                    ).with_note(
                        "For more info, see https://github.com/ablomm/ablomm-cpu/blob/main/docs/assembler/errors.md#unknown-value-of-expression",
                    )
                })?;

                Ok(((string.len() as f32) / 4.0).ceil() as u32)
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

impl Operation {
    pub fn num_words(&self) -> Result<u32, SpannedError> {
        Ok(1)
    }
}
