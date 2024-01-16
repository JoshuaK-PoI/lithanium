use crate::lang::{compiler::{statement::{ReturnStatement, StatementType}, token::{TokenStream, Span, TokenType}, CompilerResult, expression::Expression}, util::vec::UnshiftExpect};



impl ReturnStatement {
    pub(crate) fn parse(tokens: &mut TokenStream) -> CompilerResult<(StatementType, Span)> {
        let span_start = tokens.unshift_expect(TokenType::Return)?.span.start;
        let value = Expression::parse(tokens)?;
        let end = tokens.unshift_expect(TokenType::Semicolon)?;
        Ok((
            StatementType::Return(ReturnStatement { value }),
            span_start..end.span.end,
        ))
    }
}