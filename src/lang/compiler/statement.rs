use serde::Serialize;

use super::{
    expression::Expression,
    token::{Span, Token, TokenStream, TokenType},
    CompilerError, CompilerResult, ErrorCode,
};

use crate::lang::util::vec::{UnshiftExpect, Unshift};

pub(crate) type SpannedStatement = (StatementType, Span);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Statement {
    pub(crate) statement: StatementType,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) enum StatementType {
    Unknown,
    Block(BlockStatement),
    Let(LetStatement),
    Function(FunctionStatement),
    Return(ReturnStatement),
    Intrinsic(IntrinsicStatement),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct BlockStatement {
    pub(crate) statements: Vec<SpannedStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct LetStatement {
    pub(crate) name: String,
    pub(crate) value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct FunctionStatement {
    pub(crate) name: String,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) return_type: ParameterType,
    pub(crate) body: Vec<SpannedStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct IntrinsicStatement {
    pub(crate) name: String,
    pub(crate) arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) type_: ParameterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) enum ParameterType {
    Unknown,
    Integer,
    Boolean,
    String,
}

impl From<String> for ParameterType {
    fn from(string: String) -> Self {
        match string.as_str() {
            "int" => ParameterType::Integer,
            "bool" => ParameterType::Boolean,
            "string" => ParameterType::String,
            _ => ParameterType::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ReturnStatement {
    pub(crate) value: Option<Expression>,
}

impl Statement {
    pub(crate) fn parse(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        if let Some(token) = tokens.peek() {
            match token.type_ {
                TokenType::Let => Statement::let_(tokens),
                TokenType::Function => Statement::function(tokens),
                _ => Statement::statement(tokens),
            }
        } else {
            Err(CompilerError {
                error_code: ErrorCode::NoTokensLeft,
                error_message: format!("Expected start of statement, got None"),
                span_message: String::from(""),
                token: Token::invalid(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            })
        }
    }

    fn let_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::Let)?.clone();

        let name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();
        let mut value: Option<Expression> = None;

        if tokens.next_matches(TokenType::Equals) {
            // TODO: Maybe add a function that automatically unshifts when token type matches?
            tokens.unshift_expect(TokenType::Equals)?;
            value = Some(Expression::parse(tokens)?);
        }

        let end = tokens.unshift_expect(TokenType::Semicolon)?;

        Ok((
            StatementType::Let(LetStatement { name, value }),
            start.span.start..end.span.end,
        ))
    }

    fn function(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::Function)?.clone();

        let name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();

        tokens.unshift_expect(TokenType::LeftParen)?;

        let mut parameters = Vec::new();

        while !tokens.next_matches(TokenType::RightParen) {
            let parameter_name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();
            let mut parameter_type = ParameterType::Unknown;
            if tokens.next_matches(TokenType::Colon) {
                tokens.unshift_expect(TokenType::Colon)?;
                parameter_type = tokens
                    .unshift_expect(TokenType::Identifier)?
                    .value
                    .clone()
                    .into();

                if parameter_type == ParameterType::Unknown {
                    return Err(CompilerError {
                        error_code: ErrorCode::InvalidParameterType,
                        error_message: format!(
                            "Invalid parameter type: got '{}'",
                            tokens.peek().unwrap().value
                        ),
                        span_message: String::from(""),
                        token: tokens.unshift().unwrap().clone(),
                        help: Some(String::from(
                            "Expected one of: \n- int\n- bool\n- string",
                        )),
                        info: None,
                    });
                }
            }

            parameters.push(Parameter {
                name: parameter_name,
                type_: parameter_type,
            });

            if !tokens.next_matches(TokenType::RightParen) {
                tokens.unshift_expect(TokenType::Comma)?;
            }
        }

        tokens.unshift_expect(TokenType::RightParen)?;

        let mut return_type = ParameterType::Unknown;

        if tokens.next_matches(TokenType::Colon) {
            tokens.unshift_expect(TokenType::Colon)?;
            return_type = tokens
                .unshift_expect(TokenType::Identifier)?
                .value
                .clone()
                .into();

            if return_type == ParameterType::Unknown {
                return Err(CompilerError {
                    error_code: ErrorCode::InvalidReturnType,
                    error_message: format!(
                        "Invalid return type: got '{}'",
                        tokens.peek().unwrap().value
                    ),
                    span_message: String::from(""),
                    token: tokens.unshift().unwrap().clone(),
                    help: Some(String::from(
                        "Expected one of: \n- int\n- bool\n- string",
                    )),
                    info: None,
                });
            }
        }

        let body = Statement::block(tokens)?;

        Ok((
            StatementType::Function(FunctionStatement {
                name,
                parameters,
                return_type,
                body: match body.0 {
                    StatementType::Block(block) => block.statements,
                    _ => unreachable!(),
                },
            }),
            start.span.start..body.1.end,
        ))
    }

    fn statement(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        if let Some(token) = tokens.peek() {
            match token.type_ {
                TokenType::For => Statement::for_(tokens),
                TokenType::If => Statement::if_(tokens),
                TokenType::Return => Statement::return_(tokens),
                TokenType::Break => Statement::break_(tokens),
                TokenType::Continue => Statement::continue_(tokens),
                TokenType::While => Statement::while_(tokens),
                TokenType::LeftBrace => Statement::block(tokens),
                TokenType::At => Statement::intrinsic(tokens),
                _ => Statement::expression(tokens),
            }
        } else {
            Err(CompilerError {
                error_code: ErrorCode::NoTokensLeft,
                error_message: format!("Expected start of statement, got None"),
                span_message: String::from(""),
                token: Token::invalid(),
                help: Some(String::from(
                    "Expected one of: - TODO: List expected tokens",
                )),
                info: None,
            })
        }
    }

    fn for_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let mut initializer = None;
        let mut condition = None;
        let mut increment = None;

        let start = tokens.unshift_expect(TokenType::For)?.clone();

        // TODO: Differentiate between `for [init]; [cond]; [incr]` and `for [item] in [iter]`
        //       For now, only support `for [init]; [cond]; [incr]`

        if !tokens.next_matches(TokenType::Semicolon) {
            initializer = Some(Expression::parse(tokens)?);
        }

        tokens.unshift_expect(TokenType::Semicolon)?;

        if !tokens.next_matches(TokenType::Semicolon) {
            condition = Some(Expression::parse(tokens)?);
        }

        tokens.unshift_expect(TokenType::Semicolon)?;

        if !tokens.next_matches(TokenType::LeftBrace) {
            increment = Some(Expression::parse(tokens)?);
        }

        let body = Statement::block(tokens)?;

        Ok((StatementType::Unknown, start.span.start..body.1.end))
    }

    fn if_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let condition = Expression::parse(tokens)?;
        let body = Statement::block(tokens)?;

        let mut else_body = None;

        if tokens.next_matches(TokenType::Else) {
            // TODO: Support `else if` statements
            else_body = Some(Statement::block(tokens)?);
        }

        Ok((
            StatementType::Unknown,
            condition.span.start..else_body.unwrap_or(body).1.end,
        ))
    }

    fn return_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::Return)?.clone();
        let mut value = None;

        if !tokens.next_matches(TokenType::Semicolon) {
            value = Some(Expression::parse(tokens)?);
        }
        let end = tokens.unshift_expect(TokenType::Semicolon)?;

        Ok((
            StatementType::Return(ReturnStatement { value }),
            start.span.start..end.span.end,
        ))
    }

    fn break_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::Break)?.clone();
        tokens.unshift_expect(TokenType::Semicolon)?;
        Ok((StatementType::Unknown, start.span.start..start.span.end))
    }

    fn continue_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::Continue)?.clone();
        tokens.unshift_expect(TokenType::Semicolon)?;
        Ok((StatementType::Unknown, start.span.start..start.span.end))
    }

    fn while_(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::While)?.clone();
        let condition = Expression::parse(tokens)?;
        let body = Statement::block(tokens)?;

        Ok((StatementType::Unknown, start.span.start..body.1.end))
    }

    fn block(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::LeftBrace)?.span.start;

        let mut statements = Vec::new();

        while !tokens.next_matches(TokenType::RightBrace) {
            statements.push(Statement::parse(tokens)?);
        }

        let end = tokens.unshift_expect(TokenType::RightBrace)?;

        Ok((
            StatementType::Block(BlockStatement { statements }),
            start..end.span.end,
        ))
    }

    fn expression(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let expr = Expression::parse(tokens)?;
        let end = tokens.unshift_expect(TokenType::Semicolon)?;

        Ok((StatementType::Unknown, expr.span.start..end.span.end))
    }

    fn intrinsic(tokens: &mut TokenStream) -> CompilerResult<SpannedStatement> {
        let start = tokens.unshift_expect(TokenType::At)?.clone();
        let name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();
        tokens.unshift_expect(TokenType::LeftParen)?;
        let mut arguments = Vec::new();
        while !tokens.next_matches(TokenType::RightParen) {
            arguments.push(Expression::parse(tokens)?);
            if !tokens.next_matches(TokenType::RightParen) {
                tokens.unshift_expect(TokenType::Comma)?;
            }
        }
        tokens.unshift_expect(TokenType::RightParen)?;

        let end = tokens.unshift_expect(TokenType::Semicolon)?;

        Ok((
            StatementType::Intrinsic(IntrinsicStatement { name, arguments }),
            start.span.start..end.span.end,
        ))
    }
}
