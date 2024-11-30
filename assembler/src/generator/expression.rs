use super::*;

impl Spanned<&Expression> {
    pub fn eval(&self, symbol_table: &SymbolTable) -> Result<u32, Error> {
        match &self.val {
            // there is a bunch of deref's here (i.e. **a) because a and b are a Box, which has
            // it's own as_ref() function, but we really need the Spanned::as_ref() function. No
            // deref's are needed if the Spanned::as_ref() method is named differently, but I
            // didn't like that
            Expression::Number(a) => Ok(*a),
            Expression::Ident(a) => get_identifier(&Spanned::new(a, self.span), symbol_table),
            Expression::Pos(a) => (**a).as_ref().eval(symbol_table),
            Expression::Neg(a) => Ok(-((**a).as_ref().eval(symbol_table)? as i32) as u32),
            Expression::Not(a) => Ok(!(**a).as_ref().eval(symbol_table)?),
            Expression::Mul(a, b) => {
                // multiplication works with 2's compliment; no need to cast
                Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_mul((**b).as_ref().eval(symbol_table)?))
            }
            Expression::Div(a, b) => {
                let denominator = (**b).as_ref().eval(symbol_table)?;
                if denominator == 0 {
                    return Err(Error::new("divison by 0 is undefined", b.span));
                }
                Ok((**a).as_ref().eval(symbol_table)?.wrapping_div(denominator))
            }
            Expression::Remainder(a, b) => {
                let denominator = (**b).as_ref().eval(symbol_table)?;
                if denominator == 0 {
                    return Err(Error::new("divison by 0 is undefined", b.span));
                }
                Ok((**a).as_ref().eval(symbol_table)?.wrapping_rem(denominator))
            }
            Expression::Add(a, b) => Ok((**a)
                .as_ref()
                .eval(symbol_table)?
                .wrapping_add((**b).as_ref().eval(symbol_table)?)),
            Expression::Sub(a, b) => Ok((**a)
                .as_ref()
                .eval(symbol_table)?
                .wrapping_sub((**b).as_ref().eval(symbol_table)?)),
            Expression::Shl(a, b) => Ok((**a)
                .as_ref()
                .eval(symbol_table)?
                .wrapping_shl((**b).as_ref().eval(symbol_table)?)),
            Expression::Shr(a, b) => {
                // rust will use normal shift right on unsigned types
                Ok((**a)
                    .as_ref()
                    .eval(symbol_table)?
                    .wrapping_shr((**b).as_ref().eval(symbol_table)?))
            }
            Expression::Ashr(a, b) => {
                // rust will use arithmetic shift right on signed types
                Ok(((**a).as_ref().eval(symbol_table)? as i32)
                    .wrapping_shr((**b).as_ref().eval(symbol_table)?) as u32)
            }
            Expression::And(a, b) => {
                Ok((**a).as_ref().eval(symbol_table)? & (**b).as_ref().eval(symbol_table)?)
            }
            Expression::Or(a, b) => {
                Ok((**a).as_ref().eval(symbol_table)? | (**b).as_ref().eval(symbol_table)?)
            }
            Expression::Xor(a, b) => {
                Ok((**a).as_ref().eval(symbol_table)? ^ (**b).as_ref().eval(symbol_table)?)
            }
        }
    }
}
