use super::{
    expression::Expression,
    token::{Span, Token, TokenStream, TokenType},
    CompilerError, CompilerResult, ErrorCode,
};
use crate::lang::{
    structure::function::{Parameter, ParameterType},
    util::vec::{Unshift, UnshiftExpect},
};

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
    Return(ReturnStatement),
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
    pub(crate) return_type: ParameterType,
    pub(crate) body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReturnStatement {
    pub(crate) value: Expression,
}

impl Statement {
    pub(crate) fn parse(tokens: &mut TokenStream) -> CompilerResult<Statement> {
        let (type_, span) = match tokens.peek() {
            Some(token) => match token.type_ {
                TokenType::Let => LetStatement::parse(tokens),
                TokenType::Function => FunctionStatement::parse(tokens),
                TokenType::Return => ReturnStatement::parse(tokens),
                _ => Err(CompilerError {
                    error_code: ErrorCode::UnexpectedToken,
                    error_message: format!(
                        "Unexpected token {}, expected start of statement",
                        String::from(token.type_)
                    ),
                    span_message: String::from(""),
                    token: tokens.unshift().unwrap().clone(),
                    help: Some(String::from(
                        "Expected one of: - TODO: List expected tokens",
                    )),
                    info: None,
                }),
            },
            None => Err(CompilerError {
                error_code: ErrorCode::NoTokensLeft,
                error_message: format!("Expected start of statement, got None"),
                span_message: String::from(""),
                token: Token::invalid(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            }),
        }?;

        Ok(Statement { type_, span })
    }
}
