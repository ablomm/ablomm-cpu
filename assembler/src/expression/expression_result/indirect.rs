use crate::{
    error::Error,
    expression::expression_result::{AsmRef, ExpressionResult, Indirect},
    span::Spanned,
};

impl AsmRef for &Spanned<&Indirect> {
    type Output = Result<ExpressionResult, Error>;

    fn asm_ref(self) -> Self::Output {
        Ok((***self.val).clone())
    }
}
