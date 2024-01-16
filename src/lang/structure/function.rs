use crate::lang::{compiler::{statement::{FunctionStatement, StatementType, Statement}, token::{TokenStream, Span, TokenType}, CompilerResult, CompilerError, ErrorCode}, util::vec::UnshiftExpect};



#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) type_: ParameterType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParameterType {
    Unknown,
    Integer,
}

impl From<String> for ParameterType {
    fn from(string: String) -> Self {
        match string.as_str() {
            "int" => Self::Integer,
            _ => Self::Unknown,
        }
    }
}

impl FunctionStatement {
    pub(crate) fn parse(tokens: &mut TokenStream) -> CompilerResult<(StatementType, Span)> {
        let span_start = tokens.unshift_expect(TokenType::Function)?.span.start;
        let name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();

        tokens.unshift_expect(TokenType::LeftParen)?;

        let parameters = FunctionStatement::parse_parameters(tokens)?;
        
        tokens.unshift_expect(TokenType::RightParen)?;

        let mut return_type = ParameterType::Unknown;
        // Optional: Return type of function
        if let Some(_type_token) = tokens.unshift_if(TokenType::Colon) {
            let _type_token = tokens.unshift_expect(TokenType::Identifier)?;

            return_type = _type_token.value.clone().into();

            if return_type == ParameterType::Unknown {
                return Err(CompilerError {
                    error_code: ErrorCode::UnexpectedToken,
                    error_message: format!("Invalid type: got '{}'", _type_token.value),
                    span_message: String::from(""),
                    token: _type_token.clone(),
                    help: Some(String::from(
                        "Expected one of: \n- Integer literal\n- TODO: More exprected tokens",
                    )),
                    info: None,
                });
            }
        }

        tokens.unshift_expect(TokenType::LeftBrace)?;

        let mut body = Vec::new();

        while let Some(token) = tokens.peek() {
            if token.type_ == TokenType::RightBrace {
                break;
            }

            body.push(Statement::parse(tokens)?);
        }

        let end = tokens.unshift_expect(TokenType::RightBrace)?;

        Ok((
            StatementType::Function(FunctionStatement {
                name,
                parameters,
                return_type,
                body,
            }),
            span_start..end.span.end,
        ))
    }

    fn parse_parameters(
        tokens: &mut TokenStream,
    ) -> CompilerResult<Vec<Parameter>> {
        let mut parameters = Vec::new();
        while let Some(token) = tokens.peek() {
            if token.type_ == TokenType::RightParen {
                break;
            }

            let name = tokens.unshift_expect(TokenType::Identifier)?.value.clone();
            let mut type_ = ParameterType::Unknown;

            if let Some(_type_param_token) = tokens.unshift_if(TokenType::Colon) {
                let type_token = tokens.unshift_expect(TokenType::Identifier)?;

                type_ = type_token.value.clone().into();

                if type_ == ParameterType::Unknown {
                    return Err(CompilerError {
                        error_code: ErrorCode::InvalidParameterType,
                        error_message: format!("Invalid type: got '{}'", type_token.value),
                        span_message: String::from(""),
                        token: type_token.clone(),
                        help: Some(String::from(
                            "Expected one of: \n- Integer literal\n- TODO: More exprected tokens",
                        )),
                        info: None,
                    });
                }
            }

            parameters.push(Parameter { name, type_ });
            
            // The next token can be a comma, indicating another parameter
            // If it's not a comma, it must be a right paren
            if let Some(_comma_token) = tokens.unshift_if(TokenType::Comma) {
                continue;
            } else {
                break;
            }
        }
        Ok(parameters)
    }
}
