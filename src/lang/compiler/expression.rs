use serde::Serialize;

use crate::lang::util::vec::{Unshift, UnshiftExpect};

use super::{
    token::{Span, Token, TokenStream, TokenType},
    CompilerError, CompilerResult, ErrorCode,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Expression {
    pub(crate) expression: ExpressionType,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) enum ExpressionType {
    Binary(BinaryExpression),
    FunctionCall(FunctionCallExpression),
    Literal(LiteralExpression),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct BinaryExpression {
    pub(crate) left: Box<Expression>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct FunctionCallExpression {
    pub(crate) name: String,
    pub(crate) arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct LiteralExpression {
    pub(crate) literal: LiteralType,
    pub(crate) value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
        Ok(Expression::assignment(tokens)?)
    }

    fn assignment(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let mut expr = Expression::or(tokens)?;

        if tokens.next_matches(TokenType::Equals) {
            todo!()
        }

        Ok(expr)
    }

    fn or(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let mut expr = Expression::and(tokens)?;

        while tokens.next_matches(TokenType::PipePipe) {
            todo!()
        }

        Ok(expr)
    }

    fn and(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let mut expr = Expression::equality(tokens)?;

        while tokens.next_matches(TokenType::AmpersandAmpersand) {
            todo!()
        }

        Ok(expr)
    }

    fn equality(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let mut expr = Expression::comparison(tokens)?;

        while tokens.next_matches_any(&[TokenType::BangEquals, TokenType::EqualsEquals]) {
            todo!()
        }

        Ok(expr)
    }

    fn comparison(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let mut expr = Expression::term(tokens)?;

        // TODO: Add comparison operators when implementation requires it
        // while tokens.next_matches_any(&[todo!()]) {
        //     todo!()
        // }

        Ok(expr)
    }

    fn term(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        let mut expr = Expression::factor(tokens)?;

        // TODO: Add term operators when implementation requires it
        // while tokens.next_matches_any(&[todo!()]) {
        //     todo!()
        // }

        Ok(expr)
    }

    fn factor(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        // TODO: Change back to unary when implementation requires it
        let mut expr = Expression::call(tokens)?;

        while tokens.next_matches_any(&[TokenType::Asterisk, TokenType::Slash]) {
            let operator = tokens
                .unshift_expect_any(&[TokenType::Asterisk, TokenType::Slash])?
                .clone();
            // TODO: Change back to unary when implementation requires it
            let right = Expression::call(tokens)?;

            // Assign span here so `expr` can be moved into the binary expression box
            let span = expr.span.start..right.span.end;

            expr = Expression {
                expression: ExpressionType::Binary(BinaryExpression {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                }),
                span,
            };
        }

        Ok(expr)
    }

    fn unary(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        if tokens.next_matches_any(&[todo!()]) {
            todo!()
        }

        Expression::call(tokens)
    }

    fn call(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        // TODO: Change back to primary when implementation requires it
        let mut expr = Expression::literal(tokens)?;

        if tokens.next_matches(TokenType::LeftParen) {
            tokens.unshift_expect(TokenType::LeftParen)?;
            let mut arguments = Vec::new();
            while !tokens.next_matches(TokenType::RightParen) {

                arguments.push(Expression::parse(tokens)?);

                if !tokens.next_matches(TokenType::RightParen) {
                    tokens.unshift_expect(TokenType::Comma)?;
                }
            }

            tokens.unshift_expect(TokenType::RightParen)?;

            let span = expr.span.start..tokens.peek().unwrap().span.end;
            let name = match &expr.expression {
                ExpressionType::Literal(LiteralExpression { value, .. }) => value.clone(),
                _ => unreachable!(),
            };

            expr = Expression {
                expression: ExpressionType::FunctionCall(FunctionCallExpression {
                    name,
                    arguments,
                }),
                span,
            };
        }

        Ok(expr)
    }

    fn primary(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        if tokens.next_matches_any(&[todo!()]) {
            todo!()
        }

        if tokens.next_matches(TokenType::LeftParen) {
            todo!()
        }

        Expression::literal(tokens)
    }

    fn literal(tokens: &mut TokenStream) -> CompilerResult<Expression> {
        if let Some(token) = tokens.peek() {
            match token.type_ {
                TokenType::IntegerLiteral | TokenType::Identifier => {
                    let token = tokens.unshift().unwrap().clone();
                    Ok(Expression {
                        expression: ExpressionType::Literal(LiteralExpression {
                            literal: token.type_.into(),
                            value: token.value,
                        }),
                        span: token.span,
                    })
                }
                _ => Err(CompilerError {
                    error_code: ErrorCode::InvalidExpression,
                    error_message: format!("Invalid expression: got '{}'", token.value),
                    span_message: String::from(""),
                    token: tokens.unshift().unwrap().clone(),
                    help: Some(String::from(
                        "Expected one of: \n- Integer literal\n- TODO: More exprected tokens",
                    )),
                    info: None,
                }),
            }
        } else {
            Err(CompilerError {
                error_code: ErrorCode::UnexpectedToken,
                error_message: format!("End of file reached while parsing expression"),
                span_message: String::from(""),
                token: Token::invalid(), // TODO: Store the last token somewhere so we can use it in error messages
                help: Some(String::from(
                    "Expected an expression, but reached the end of the file",
                )),
                info: None,
            })
        }
    }
}
