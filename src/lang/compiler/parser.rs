use super::{lexer::Token, CompilerResult, CompilerError, ErrorCode};

#[derive(Debug, Clone)]
pub(crate) struct AST {}

#[derive(Debug, Clone)]
pub(crate) struct Parser {}


impl Parser {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse<'a>(&'a mut self, tokens: &'a mut Vec<Token>) -> CompilerResult<AST> {

        while let Some(token) = tokens.iter().next() {
            match token.type_ {
                _ => {
                    return Err(CompilerError {
                        error_code: ErrorCode::UnexpectedToken,
                        error_message: String::from("Unexpected token"),
                        span_message: String::from(""),
                        token,
                        help: Some(String::from("Expected one of: - TODO: List expected tokens")),
                        info: None,
                    });
                }
            }
        }

        Ok(AST {})
    }
}