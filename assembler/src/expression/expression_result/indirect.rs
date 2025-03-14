use super::*;

impl AsmRef for &Spanned<&Indirect> {
    type Output = Result<ExpressionResult, SpannedError>;

    fn asm_ref(self) -> Self::Output {
        Ok((***self.val).clone())
    }
}
