use crate::lang::{compiler::{statement::{LetStatement, StatementType}, token::{TokenStream, Span, TokenType}, CompilerResult, expression::Expression}, util::vec::UnshiftExpect};

impl LetStatement {
    pub(crate) fn parse(tokens: &mut TokenStream) -> CompilerResult<(StatementType, Span)> {
        let span_start = tokens.unshift_expect(TokenType::Let)?.span.start;
        let name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();
        tokens.unshift_expect(TokenType::Equals)?;
        let value = Expression::parse(tokens)?;
        let end = tokens.unshift_expect(TokenType::Semicolon)?;
        Ok((
            StatementType::Let(LetStatement { name, value }),
            span_start..end.span.end,
        ))
    }
}
