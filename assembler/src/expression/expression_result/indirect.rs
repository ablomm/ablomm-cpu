use super::*;

impl AsmRef for &Spanned<&Indirect> {
    type Output = Result<Spanned<ExpressionResult>, Error>;

    fn asm_ref(self) -> Self::Output {
        Ok(Spanned::new((***self.val).clone(), self.span))
    }
}
