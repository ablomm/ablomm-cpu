use super::*;

#[derive(Debug, Clone)]
pub enum ExpressionResult {
    Number(u32),
    String(String),
    Register(Register),
    RegisterOffset(Spanned<Register>, u32),
    Indirect(Box<ExpressionResult>),
}

impl Spanned<&ExpressionResult> {
    pub fn pos(&self) -> Result<Spanned<ExpressionResult>, Error> {
        Ok(Spanned::new(self.val.clone(), self.span))
    }

    pub fn neg(&self) -> Result<Spanned<ExpressionResult>, Error> {
        if let ExpressionResult::Number(a) = self.val {
            return Ok(Spanned::new(
                ExpressionResult::Number(-(*a as i32) as u32),
                self.span,
            ));
        }
        Err(Error::new(
            format!("Expected {}", "number".fg(ATTENTION_COLOR)),
            self.span,
        ))
    }

    pub fn not(&self) -> Result<Spanned<ExpressionResult>, Error> {
        if let ExpressionResult::Number(a) = self.val {
            return Ok(Spanned::new(ExpressionResult::Number(!*a), self.span));
        }
        Err(Error::new(
            format!("Expected {}", "number".fg(ATTENTION_COLOR)),
            self.span,
        ))
    }

    pub fn mul(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number(a.wrapping_mul(b)),
            self.span.union(&other.span),
        ))
    }

    pub fn div(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        if b == 0 {
            return Err(Error::new("Divison by 0 is undefined", other.span));
        }
        Ok(Spanned::new(
            ExpressionResult::Number(a.wrapping_div(b)),
            self.span.union(&other.span),
        ))
    }

    pub fn remainder(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        if b == 0 {
            return Err(Error::new("Divison by 0 is undefined", other.span));
        }
        Ok(Spanned::new(
            ExpressionResult::Number(a.wrapping_rem(b)),
            self.span.union(&other.span),
        ))
    }
    // add has a lot of functionality / operator overloads
    // for numbers, addition
    // for strings, concationation,
    // for register offsets, adding offsets
    pub fn add(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        match self.val {
            ExpressionResult::Number(a) => match other.val {
                ExpressionResult::Number(b) => Ok(Spanned::new(
                    ExpressionResult::Number(a.wrapping_add(*b)),
                    self.span.union(&other.span),
                )),
                ExpressionResult::String(b) => Ok(Spanned::new(
                    ExpressionResult::String(a.to_string() + b),
                    self.span.union(&other.span),
                )),
                ExpressionResult::Register(b) => {
                    if *a == 0 {
                        return Ok(Spanned::new(
                            ExpressionResult::Register(*b),
                            self.span.union(&other.span),
                        ));
                    }
                    Ok(Spanned::new(
                        ExpressionResult::RegisterOffset(Spanned::new(*b, self.span), *a),
                        self.span.union(&other.span),
                    ))
                }
                ExpressionResult::RegisterOffset(b_reg, b_offset) => {
                    let new_offset = a.wrapping_add(*b_offset);

                    if new_offset == 0 {
                        return Ok(Spanned::new(
                            ExpressionResult::Register(b_reg.val),
                            self.span.union(&other.span),
                        ));
                    }

                    Ok(Spanned::new(
                        ExpressionResult::RegisterOffset(*b_reg, a.wrapping_add(*b_offset)),
                        self.span.union(&other.span),
                    ))
                }
                _ => Err(Error::new(
                    format!(
                        "Invalid RHS type, expected either {}, {}, {}, or {}",
                        "number".fg(ATTENTION_COLOR),
                        "string".fg(ATTENTION_COLOR),
                        "register".fg(ATTENTION_COLOR),
                        "register offset".fg(ATTENTION_COLOR),
                    ),
                    other.span,
                )),
            },
            ExpressionResult::String(a) => match other.val {
                ExpressionResult::Number(b) => Ok(Spanned::new(
                    ExpressionResult::String(a.to_string() + &b.to_string()),
                    self.span.union(&other.span),
                )),
                ExpressionResult::String(b) => Ok(Spanned::new(
                    ExpressionResult::String(a.to_string() + b),
                    self.span.union(&other.span),
                )),
                _ => Err(Error::new(
                    format!(
                        "Invalid RHS type, expected either {} or {}",
                        "number".fg(ATTENTION_COLOR),
                        "string".fg(ATTENTION_COLOR)
                    ),
                    other.span,
                )),
            },
            ExpressionResult::Register(a) => Spanned::new(
                &ExpressionResult::RegisterOffset(Spanned::new(*a, self.span), 0),
                self.span,
            )
            .add(other),
            ExpressionResult::RegisterOffset(a_reg, a_offset) => match other.val {
                ExpressionResult::Number(b) => {
                    let new_offset = a_offset.wrapping_add(*b);
                    if new_offset == 0 {
                        return Ok(Spanned::new(
                            ExpressionResult::Register(a_reg.val),
                            self.span.union(&other.span),
                        ));
                    }
                    Ok(Spanned::new(
                        ExpressionResult::RegisterOffset(*a_reg, new_offset),
                        self.span.union(&other.span),
                    ))
                }
                _ => Err(Error::new(
                    format!(
                        "Invalid RHS type, expected {}",
                        "number".fg(ATTENTION_COLOR)
                    ),
                    other.span,
                )),
            },
            _ => Err(Error::new(
                format!(
                    "Invalid LHS type, expected either {}, {}, {}, or {}",
                    "number".fg(ATTENTION_COLOR),
                    "string".fg(ATTENTION_COLOR),
                    "register".fg(ATTENTION_COLOR),
                    "register offset".fg(ATTENTION_COLOR)
                ),
                self.span,
            )),
        }
    }

    pub fn sub(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        match self.val {
            ExpressionResult::Number(a) => match other.val {
                ExpressionResult::Number(b) => Ok(Spanned::new(
                    ExpressionResult::Number(a.wrapping_sub(*b)),
                    self.span.union(&other.span),
                )),
                ExpressionResult::Register(b) => {
                    if *a == 0 {
                        return Ok(Spanned::new(
                            ExpressionResult::Register(*b),
                            self.span.union(&other.span),
                        ));
                    }
                    Ok(Spanned::new(
                        ExpressionResult::RegisterOffset(
                            Spanned::new(*b, other.span),
                            -(*a as i32) as u32,
                        ),
                        self.span.union(&other.span),
                    ))
                }
                ExpressionResult::RegisterOffset(b_reg, b_offset) => {
                    let new_offset = a.wrapping_sub(*b_offset);

                    if new_offset == 0 {
                        return Ok(Spanned::new(
                            ExpressionResult::Register(b_reg.val),
                            other.span,
                        ));
                    }

                    Ok(Spanned::new(
                        ExpressionResult::RegisterOffset(*b_reg, a.wrapping_sub(*b_offset)),
                        self.span.union(&other.span),
                    ))
                }
                _ => Err(Error::new(
                    format!(
                        "Invalid RHS type, expected either {}, {}, or {}",
                        "number".fg(ATTENTION_COLOR),
                        "register".fg(ATTENTION_COLOR),
                        "register offset".fg(ATTENTION_COLOR),
                    ),
                    other.span,
                )),
            },
            ExpressionResult::Register(a) => Spanned::new(
                &ExpressionResult::RegisterOffset(Spanned::new(*a, self.span), 0),
                self.span,
            )
            .sub(other),
            ExpressionResult::RegisterOffset(a_reg, a_offset) => match other.val {
                ExpressionResult::Number(b) => {
                    let new_offset = a_offset.wrapping_sub(*b);
                    if new_offset == 0 {
                        return Ok(Spanned::new(
                            ExpressionResult::Register(a_reg.val),
                            self.span,
                        ));
                    }
                    Ok(Spanned::new(
                        ExpressionResult::RegisterOffset(*a_reg, new_offset),
                        self.span.union(&other.span),
                    ))
                }
                _ => Err(Error::new(
                    format!(
                        "Invalid RHS type, expected {}",
                        "number".fg(ATTENTION_COLOR)
                    ),
                    other.span,
                )),
            },
            _ => Err(Error::new(
                format!(
                    "Invalid LHS type, expected either {}, {}, or {}",
                    "number".fg(ATTENTION_COLOR),
                    "register".fg(ATTENTION_COLOR),
                    "register offset".fg(ATTENTION_COLOR)
                ),
                self.span,
            )),
        }
    }

    pub fn shl(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number(a.wrapping_shl(b)),
            self.span.union(&other.span),
        ))
    }

    pub fn shr(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number(a.wrapping_shr(b)),
            self.span.union(&other.span),
        ))
    }

    pub fn ashr(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number((a as i32).wrapping_shr(b) as u32),
            self.span.union(&other.span),
        ))
    }

    pub fn and(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number(a & b),
            self.span.union(&other.span),
        ))
    }

    pub fn or(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number(a | b),
            self.span.union(&other.span),
        ))
    }

    pub fn xor(&self, other: &Self) -> Result<Spanned<ExpressionResult>, Error> {
        let (a, b) = parse_num_num(self, other)?;
        Ok(Spanned::new(
            ExpressionResult::Number(a ^ b),
            self.span.union(&other.span),
        ))
    }
}

fn parse_num_num(
    a: &Spanned<&ExpressionResult>,
    b: &Spanned<&ExpressionResult>,
) -> Result<(u32, u32), Error> {
    if let ExpressionResult::Number(a) = a.val {
        if let ExpressionResult::Number(b) = b.val {
            Ok((*a, *b))
        } else {
            Err(Error::new(
                format!("Expected {}", "number".fg(ATTENTION_COLOR)),
                b.span,
            ))
        }
    } else {
        Err(Error::new(
            format!("Expected {}", "number".fg(ATTENTION_COLOR)),
            a.span,
        ))
    }
}
