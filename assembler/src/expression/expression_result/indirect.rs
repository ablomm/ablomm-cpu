use super::*;

impl AsmRef for &Spanned<&Indirect> {
    type Output = Result<ExpressionResult, Error>;

    fn asm_ref(self) -> Self::Output {
        Ok((***self.val).clone())
    }
}
