use crate::lang::util::vec::Unshift;

use super::{token::{Span, Token, TokenType, TokenStream}, CompilerResult, CompilerError, ErrorCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Expression {
    pub(crate) type_: ExpressionType,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExpressionType {
    Literal(LiteralExpression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LiteralExpression {
    pub(crate) type_: LiteralType,
    pub(crate) value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LiteralType {
    Unknown,
    Integer,
}

impl Into<LiteralType> for TokenType {
    fn into(self) -> LiteralType {
        match self {
            TokenType::IntegerLiteral => LiteralType::Integer,
            _ => LiteralType::Unknown,
        }
    }

}

impl Expression {
    pub(crate) fn parse(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let expression = Expression::parse_literal(tokens)?;

        // TODO: Create a recursive descent parser for expressions

        Ok(expression)
    }

    fn parse_literal(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let literal_token = match tokens.peek() {
            Some(token) if token.type_ == TokenType::IntegerLiteral => tokens.unshift().unwrap(),
            Some(token) => {
                return Err(CompilerError {
                    error_code: ErrorCode::UnexpectedToken,
                    error_message: format!("Invalid expression: got '{}'", token.value),
                    span_message: String::from(""),
                    token: tokens.unshift().unwrap().clone(),
                    help: Some(String::from("Expected one of: \n- Integer literal\n- TODO: More exprected tokens")),
                    info: None,
                });
            }
            None => {
                return Err(CompilerError {
                    error_code: ErrorCode::UnexpectedToken,
                    error_message: format!("End of file reached while parsing expression"),
                    span_message: String::from(""),
                    token: Token::invalid(), // TODO: Store the last token somewhere so we can use it in error messages
                    help: Some(String::from("Expected an expression, but reached the end of the file")),
                    info: None,
                });
            }
        };

        return Ok(Expression {
            type_: ExpressionType::Literal(LiteralExpression {type_: literal_token.type_.into(), value: literal_token.value.clone()}),
            span: literal_token.span.clone(), // TODO: When expressions span multiple tokens, this needs to be updated to reflect the entire expression span
        });

    }
}