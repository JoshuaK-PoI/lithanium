use crate::lang::util::vec::{UnshiftExpect, Unshift};

use super::{token::{Span, Token, TokenType}, CompilerResult, CompilerError, ErrorCode};

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
    pub(crate) fn parse(tokens: &mut Vec<Token>) -> CompilerResult<Expression> {
        let expression = Expression::parse_literal(tokens)?;

        Ok(expression)
    }

    fn parse_literal(tokens: &mut Vec<Token>) -> CompilerResult<Expression> {
        let literal_token = match tokens.unshift() {
            Some(token) if token.type_ == TokenType::IntegerLiteral => token,
            Some(token) => {
                return Err(CompilerError {
                    error_code: ErrorCode::UnexpectedToken,
                    error_message: format!("Invalid expression: got '{}'", token.value),
                    span_message: String::from(""),
                    token,
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
            type_: ExpressionType::Literal(LiteralExpression {type_: literal_token.type_.into(), value: literal_token.value}),
            span: literal_token.span, // TODO: When expressions span multiple tokens, this needs to be updated to reflect the entire expression span
        });

    }
}