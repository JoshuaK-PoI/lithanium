use crate::lang::{util::vec::UnshiftExpect, structure::function::Parameter};

use super::{token::{Span, Token, TokenType}, expression::Expression, CompilerResult, CompilerError, ErrorCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Statement {
    pub(crate) type_: StatementType,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StatementType {
    Unknown,
    Let(LetStatement),
    Function(FunctionStatement),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LetStatement {
    pub(crate) name: String,
    pub(crate) value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionStatement {
    pub(crate) name: String,
    pub(crate) parameters: Vec<Parameter>,

}

impl Statement {
    pub(crate) fn parse(token: Token, tokens: &mut Vec<Token>) -> CompilerResult<Statement> {
        let start_span = token.span.start;

        let (type_, end_span) = match token.type_ {
            TokenType::Let => LetStatement::parse(tokens),
            TokenType::Function => FunctionStatement::parse(tokens),
            _ => Err(CompilerError {
                    error_code: ErrorCode::UnexpectedToken,
                    error_message: format!("Unexpected token {}", String::from(token.type_)),
                    span_message: String::from(""),
                    token,
                    help: Some(String::from("Expected one of: - TODO: List expected tokens")),
                    info: None,
                })
        }?;

        Ok(Statement {
            type_,
            span: start_span..end_span
        })
    }
}

impl LetStatement {
    pub(crate) fn parse(tokens: &mut Vec<Token>) -> CompilerResult<(StatementType, usize)> {
        let name = tokens.unshift_expect(TokenType::Identifier)?.value;
        tokens.unshift_expect(TokenType::Equals)?;
        let value = Expression::parse(tokens)?;
        let end = tokens.unshift_expect(TokenType::Semicolon)?;
        Ok((StatementType::Let(LetStatement {
            name,
            value,
        }), end.span.end))
    }
}

impl FunctionStatement {
    pub(crate) fn parse(tokens: &mut Vec<Token>) -> CompilerResult<(StatementType, usize)> {

        // TODO: Peekable unshift_checked
        todo!()

    }
}